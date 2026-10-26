use std::{
    collections::VecDeque,
    env,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use reqwest::Client;
use serde::Serialize;
use serde_json::{Map, Value};
use tokio::{sync::Notify, task::JoinHandle, time::MissedTickBehavior};

use super::event::{Event, FinishedEvent};

const DEFAULT_FLUSH_MS: u64 = 1_000;
const DEFAULT_MAX_BATCH: usize = 1_000;
const DEFAULT_MAX_BYTES: usize = 1024 * 1024;
const DEFAULT_QUEUE_CAPACITY: usize = 65_536;
const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_SHUTDOWN_TIMEOUT_MS: u64 = 10_000;

/// Limits reported by Acta's `POST /api/ingest`.
const MAX_BATCH: usize = 10_000;
const MAX_BYTES: usize = 16 * 1024 * 1024;

const MAX_BACKOFF: Duration = Duration::from_secs(30);
const DROP_LOG_INTERVAL: Duration = Duration::from_secs(30);

/// Client-side settings for the Acta exporter.
#[derive(Clone, Debug)]
pub struct Config {
    pub endpoint: String,
    pub source: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub flush_interval: Duration,
    pub max_batch: usize,
    pub max_bytes: usize,
    pub queue_capacity: usize,
    pub request_timeout: Duration,
    pub shutdown_timeout: Duration,
}

impl Config {
    /// Read the configuration from the environment.
    ///
    /// `Ok(None)` means export is disabled: `ACTA_URL` is unset, or `ACTA_INGEST_AUTH`
    /// is on and the Acta sensor credentials are not available. The site still
    /// serves either way.
    pub fn from_env() -> Result<Option<Self>, String> {
        let Some(host) = env::var("ACTA_URL")
            .ok()
            .map(|host| host.trim().to_string())
            .filter(|host| !host.is_empty())
        else {
            eprintln!("wide: ACTA_URL is not set; request event export is disabled");
            return Ok(None);
        };
        if !host.starts_with("http://") {
            return Err(format!(
                "ACTA_URL must be an http:// URL (this build has no TLS client): {host}"
            ));
        }

        let endpoint = ingest_endpoint(&host);
        // `ACTA_INGEST_AUTH=off` is for a local Acta whose authentication is
        // disabled. It is deliberately explicit: with auth on (the default), a
        // missing credential disables export rather than sending requests Acta
        // would reject.
        let auth = env_flag("ACTA_INGEST_AUTH", true)?;
        let (user, password) = if auth {
            let user = env::var("ACTA_INGEST_USER")
                .ok()
                .filter(|value| !value.is_empty());
            let password = env::var("ACTA_INGEST_PASSWORD")
                .ok()
                .filter(|value| !value.is_empty());
            // The site must keep serving when Acta's sensor credential is not
            // installed (for example before Acta's deploy:secrets has run).
            match (user, password) {
                (Some(user), Some(password)) => (Some(user), Some(password)),
                (None, None) => {
                    eprintln!(
                        "wide: ACTA_INGEST_USER/ACTA_INGEST_PASSWORD are not set; request event export is disabled"
                    );
                    return Ok(None);
                }
                _ => {
                    eprintln!(
                        "wide: only one of ACTA_INGEST_USER/ACTA_INGEST_PASSWORD is set; request event export is disabled"
                    );
                    return Ok(None);
                }
            }
        } else {
            // Ignore any credentials that happened to be in the environment.
            (None, None)
        };

        let source = env::var("ACTA_SOURCE")
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "marending".to_string());
        if source.len() > 256 || source.chars().any(char::is_control) {
            return Err("ACTA_SOURCE must be at most 256 bytes without control characters".into());
        }

        let flush_interval = Duration::from_millis(env_u64("ACTA_FLUSH_MS", DEFAULT_FLUSH_MS)?);
        if flush_interval.is_zero() {
            return Err("ACTA_FLUSH_MS must be greater than zero".into());
        }

        let max_batch = env_usize("ACTA_MAX_BATCH", DEFAULT_MAX_BATCH)?;
        if !(1..=MAX_BATCH).contains(&max_batch) {
            return Err(format!("ACTA_MAX_BATCH must be between 1 and {MAX_BATCH}"));
        }

        let max_bytes = env_usize("ACTA_MAX_BYTES", DEFAULT_MAX_BYTES)?;
        if !(1..=MAX_BYTES).contains(&max_bytes) {
            return Err(format!("ACTA_MAX_BYTES must be between 1 and {MAX_BYTES}"));
        }

        let queue_capacity = env_usize("ACTA_QUEUE_CAPACITY", DEFAULT_QUEUE_CAPACITY)?;
        if queue_capacity == 0 {
            return Err("ACTA_QUEUE_CAPACITY must be at least 1".into());
        }

        let request_timeout = Duration::from_millis(env_u64(
            "ACTA_REQUEST_TIMEOUT_MS",
            DEFAULT_REQUEST_TIMEOUT_MS,
        )?);
        if request_timeout.is_zero() {
            return Err("ACTA_REQUEST_TIMEOUT_MS must be greater than zero".into());
        }

        let shutdown_timeout = Duration::from_millis(env_u64(
            "ACTA_SHUTDOWN_TIMEOUT_MS",
            DEFAULT_SHUTDOWN_TIMEOUT_MS,
        )?);
        if shutdown_timeout.is_zero() {
            return Err("ACTA_SHUTDOWN_TIMEOUT_MS must be greater than zero".into());
        }

        eprintln!(
            "wide: exporting to {endpoint} as {source:?} (flush {} ms, batch {max_batch} / {max_bytes} bytes, queue {queue_capacity}, auth {})",
            flush_interval.as_millis(),
            if user.is_some() { "on" } else { "off" },
        );

        Ok(Some(Self {
            endpoint,
            source,
            user,
            password,
            flush_interval,
            max_batch,
            max_bytes,
            queue_capacity,
            request_timeout,
            shutdown_timeout,
        }))
    }
}

/// The running exporter task, and the handle request middleware uses.
pub struct Exporter {
    sink: Sink,
    shutdown: Arc<Notify>,
    task: JoinHandle<()>,
    shutdown_timeout: Duration,
}

impl Exporter {
    /// Start from the environment, or return `None` when export is disabled.
    pub fn from_env() -> Result<Option<Self>, String> {
        let Some(config) = Config::from_env()? else {
            return Ok(None);
        };
        Self::start(config).map(Some)
    }

    pub fn start(config: Config) -> Result<Self, String> {
        let client = Client::builder()
            .timeout(config.request_timeout)
            .build()
            .map_err(|error| format!("could not build the Acta HTTP client: {error}"))?;

        let buffer = Arc::new(Buffer::new(config.queue_capacity));
        let closed = Arc::new(AtomicBool::new(false));
        let shutdown = Arc::new(Notify::new());
        let sink = Sink {
            source: Arc::from(config.source.as_str()),
            buffer: Arc::clone(&buffer),
            closed: Arc::clone(&closed),
            last_drop_log: Arc::new(Mutex::new(Instant::now())),
        };
        let task = tokio::spawn(run(
            config.clone(),
            client,
            buffer,
            closed,
            Arc::clone(&shutdown),
        ));

        Ok(Self {
            sink,
            shutdown,
            task,
            shutdown_timeout: config.shutdown_timeout,
        })
    }

    pub fn sink(&self) -> Sink {
        self.sink.clone()
    }

    /// Stop accepting events, drain the queue, and wait for the task to finish.
    pub async fn shutdown(self) {
        self.sink.closed.store(true, Ordering::SeqCst);
        self.shutdown.notify_one();
        if tokio::time::timeout(self.shutdown_timeout, self.task)
            .await
            .is_err()
        {
            eprintln!(
                "wide: exporter did not finish within {} ms",
                self.shutdown_timeout.as_millis()
            );
        }
    }
}

/// A cheap, cloneable handle to the request event buffer.
#[derive(Clone)]
pub struct Sink {
    source: Arc<str>,
    buffer: Arc<Buffer>,
    closed: Arc<AtomicBool>,
    last_drop_log: Arc<Mutex<Instant>>,
}

impl Sink {
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Close the event and append it to the buffer.
    ///
    /// This never waits on the network or on the exporter. When the buffer is
    /// full the oldest event is overwritten, so a restarted Acta loses the
    /// oldest data rather than all data after the queue fills.
    pub fn send(&self, event: Event) {
        if self.closed.load(Ordering::Relaxed) {
            return;
        }
        let Some(event) = event.finish() else {
            return;
        };
        if let Some(total) = self.buffer.push(event) {
            let mut last = self
                .last_drop_log
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if last.elapsed() >= DROP_LOG_INTERVAL {
                *last = Instant::now();
                eprintln!(
                    "wide: event queue full; dropped the oldest event ({total} dropped so far)"
                );
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn take_events(&self) -> Vec<FinishedEvent> {
        self.buffer.drain(usize::MAX, usize::MAX)
    }
}

/// The bounded, newest-wins buffer shared by the request path and the exporter.
struct Buffer {
    events: Mutex<VecDeque<FinishedEvent>>,
    capacity: usize,
    dropped: AtomicU64,
}

impl Buffer {
    fn new(capacity: usize) -> Self {
        Self {
            events: Mutex::new(VecDeque::with_capacity(capacity.min(1024))),
            capacity,
            dropped: AtomicU64::new(0),
        }
    }

    /// Push a finished event, dropping the oldest one if the queue is full.
    /// Returns the total number of dropped events when a drop happened.
    fn push(&self, event: FinishedEvent) -> Option<u64> {
        let mut events = self
            .events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut dropped = None;
        if events.len() >= self.capacity {
            events.pop_front();
            dropped = Some(self.dropped.fetch_add(1, Ordering::Relaxed) + 1);
        }
        events.push_back(event);
        dropped
    }

    /// Put a failed batch back at the front, then trim the oldest events if
    /// new arrivals filled the room meanwhile. Returns how many were dropped.
    fn requeue(&self, batch: Vec<FinishedEvent>) -> u64 {
        let mut events = self
            .events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        for event in batch.into_iter().rev() {
            events.push_front(event);
        }
        let mut dropped = 0;
        while events.len() > self.capacity {
            events.pop_front();
            dropped += 1;
        }
        self.dropped.fetch_add(dropped, Ordering::Relaxed);
        dropped
    }

    fn drain(&self, max_batch: usize, max_bytes: usize) -> Vec<FinishedEvent> {
        let mut events = self
            .events
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut batch = Vec::new();
        let mut bytes: usize = 0;
        while batch.len() < max_batch {
            let Some(event) = events.pop_front() else {
                break;
            };
            if !batch.is_empty() && bytes.saturating_add(event.size) > max_bytes {
                events.push_front(event);
                break;
            }
            bytes = bytes.saturating_add(event.size);
            batch.push(event);
        }
        batch
    }
}

async fn run(
    config: Config,
    client: Client,
    buffer: Arc<Buffer>,
    closed: Arc<AtomicBool>,
    shutdown: Arc<Notify>,
) {
    let mut ticker = tokio::time::interval(config.flush_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    ticker.tick().await; // the first tick fires immediately
    let mut backoff = Duration::ZERO;
    let mut next_attempt = Instant::now();

    loop {
        tokio::select! {
            _ = shutdown.notified() => break,
            _ = ticker.tick() => {
                if Instant::now() < next_attempt {
                    continue;
                }
                let batch = buffer.drain(config.max_batch, config.max_bytes);
                if batch.is_empty() {
                    continue;
                }
                match send_batch(&client, &config, &batch).await {
                    Ok(()) => {
                        backoff = Duration::ZERO;
                        next_attempt = Instant::now();
                    }
                    Err(error) => {
                        let count = batch.len();
                        eprintln!("wide: {error}; keeping {count} event(s) buffered");
                        let dropped = buffer.requeue(batch);
                        if dropped > 0 {
                            eprintln!(
                                "wide: event queue full; dropped {dropped} oldest event(s)"
                            );
                        }
                        backoff = if backoff.is_zero() {
                            Duration::from_secs(1)
                        } else {
                            (backoff * 2).min(MAX_BACKOFF)
                        };
                        next_attempt = Instant::now() + backoff;
                    }
                }
            }
        }
    }

    closed.store(true, Ordering::SeqCst);
    loop {
        let batch = buffer.drain(config.max_batch, config.max_bytes);
        if batch.is_empty() {
            break;
        }
        if let Err(error) = send_batch(&client, &config, &batch).await {
            eprintln!(
                "wide: final flush failed: {error}; dropping {} event(s)",
                batch.len()
            );
            break;
        }
    }
}

#[derive(Serialize)]
struct IngestItem<'a> {
    source: &'a str,
    timestamp: i64,
    data: &'a Map<String, Value>,
}

async fn send_batch(
    client: &Client,
    config: &Config,
    batch: &[FinishedEvent],
) -> Result<(), String> {
    let items: Vec<IngestItem<'_>> = batch
        .iter()
        .map(|event| IngestItem {
            source: &event.source,
            timestamp: event.timestamp_us,
            data: &event.data,
        })
        .collect();

    let mut request = client.post(&config.endpoint).json(&items);
    if let (Some(user), Some(password)) = (&config.user, &config.password) {
        request = request.basic_auth(user, Some(password));
    }

    let response = request
        .send()
        .await
        .map_err(|error| format!("POST {} failed: {error}", config.endpoint))?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        let body = body.chars().take(300).collect::<String>();
        return Err(format!(
            "POST {} returned {status}: {body}",
            config.endpoint
        ));
    }
    Ok(())
}

fn ingest_endpoint(host: &str) -> String {
    let host = host.trim_end_matches('/');
    if host.ends_with("/api/ingest") {
        host.to_string()
    } else {
        format!("{host}/api/ingest")
    }
}

fn env_u64(name: &str, default: u64) -> Result<u64, String> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => value
            .trim()
            .parse()
            .map_err(|error| format!("{name} is not a number: {error}")),
        _ => Ok(default),
    }
}

fn env_usize(name: &str, default: usize) -> Result<usize, String> {
    let value = env_u64(name, default as u64)?;
    usize::try_from(value).map_err(|_| format!("{name} is too large"))
}

fn env_flag(name: &str, default: bool) -> Result<bool, String> {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => match value.trim().to_ascii_lowercase().as_str() {
            "on" | "true" | "1" => Ok(true),
            "off" | "false" | "0" => Ok(false),
            other => Err(format!("{name} must be on or off, got {other:?}")),
        },
        _ => Ok(default),
    }
}

#[cfg(test)]
pub(crate) fn test_sink(capacity: usize) -> Sink {
    Sink {
        source: Arc::from("test"),
        buffer: Arc::new(Buffer::new(capacity)),
        closed: Arc::new(AtomicBool::new(false)),
        last_drop_log: Arc::new(Mutex::new(Instant::now())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Bytes, http::StatusCode, routing::post, Router};
    use serde_json::json;
    use tokio::sync::mpsc;

    #[test]
    fn endpoint_accepts_a_base_url_or_the_full_path() {
        assert_eq!(
            ingest_endpoint("http://127.0.0.1:8080"),
            "http://127.0.0.1:8080/api/ingest"
        );
        assert_eq!(
            ingest_endpoint("http://127.0.0.1:8080/"),
            "http://127.0.0.1:8080/api/ingest"
        );
        assert_eq!(
            ingest_endpoint("http://127.0.0.1:8080/api/ingest"),
            "http://127.0.0.1:8080/api/ingest"
        );
    }

    #[tokio::test]
    async fn exporter_batches_events_into_one_post() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let (tx, mut rx) = mpsc::channel::<Bytes>(4);
        let server = tokio::spawn(async move {
            let app = Router::new().route(
                "/api/ingest",
                post(move |body: Bytes| {
                    let tx = tx.clone();
                    async move {
                        tx.send(body).await.unwrap();
                        StatusCode::ACCEPTED
                    }
                }),
            );
            axum::serve(listener, app).await.unwrap();
        });

        let exporter = Exporter::start(Config {
            endpoint: format!("http://{address}/api/ingest"),
            source: "test".into(),
            user: None,
            password: None,
            flush_interval: Duration::from_millis(10),
            max_batch: 10,
            max_bytes: 1024,
            queue_capacity: 10,
            request_timeout: Duration::from_secs(1),
            shutdown_timeout: Duration::from_secs(2),
        })
        .unwrap();

        let sink = exporter.sink();
        for index in 0..2 {
            let event = Event::new("test");
            event.add("index", index);
            sink.send(event);
        }

        let body = tokio::time::timeout(Duration::from_secs(2), rx.recv())
            .await
            .expect("batch arrived")
            .expect("channel open");
        let payload: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(payload.as_array().unwrap().len(), 2);
        assert_eq!(payload[0]["source"], "test");
        assert_eq!(payload[0]["data"]["index"], json!(0));
        assert!(payload[0]["timestamp"].as_i64().unwrap() > 1_000_000_000_000_000);

        exporter.shutdown().await;
        server.abort();
    }
}
