use std::future::Future;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serde_json::{json, Map, Value};

tokio::task_local! {
    /// The event of the request this task is handling, if any.
    static CURRENT: Event;
}

/// A request-scoped JSON object.
///
/// Cloning the handle is cheap; every clone writes to the same object.
#[derive(Clone)]
pub struct Event {
    inner: Arc<Inner>,
}

struct Inner {
    source: String,
    timestamp_us: i64,
    /// `None` once the event has been handed to the exporter. Holding the
    /// snapshot in the same lock makes closing the event race-free: a writer
    /// either sees an open event or sees that it is closed.
    fields: Mutex<Option<Map<String, Value>>>,
}

/// The snapshot handed to the exporter. No fields can be added after this.
pub(crate) struct FinishedEvent {
    pub(crate) source: String,
    pub(crate) timestamp_us: i64,
    pub(crate) data: Map<String, Value>,
    /// Approximate serialized size, used to bound a batch.
    pub(crate) size: usize,
}

impl Event {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(Inner {
                source: source.into(),
                timestamp_us: now_us(),
                fields: Mutex::new(Some(Map::new())),
            }),
        }
    }

    /// Set a top-level field. Last write wins. A finished event ignores writes.
    pub fn add<T: Serialize>(&self, key: &str, value: T) {
        match serde_json::to_value(value) {
            Ok(value) => self.insert(key, value),
            Err(error) => self.error("serialize", format!("{key}: {error}")),
        }
    }

    /// Set an already-built JSON value. Last write wins.
    pub fn insert(&self, key: &str, value: Value) {
        let mut fields = self.fields();
        if let Some(fields) = fields.as_mut() {
            fields.insert(key.to_string(), value);
        }
    }

    /// Flatten a serializable object into this event. Last write wins.
    #[allow(dead_code)] // public event API for request-path code
    pub fn merge<T: Serialize>(&self, value: T) {
        match serde_json::to_value(value) {
            Ok(Value::Object(object)) => {
                let mut fields = self.fields();
                if let Some(fields) = fields.as_mut() {
                    fields.extend(object);
                }
            }
            Ok(other) => self.error("merge", format!("expected an object, got {other}")),
            Err(error) => self.error("serialize", error.to_string()),
        }
    }

    /// Append `{"event": ..., "error": ...}` to the `errors` array.
    ///
    /// The shape matches what the old tracing instrumentation produced, so the
    /// Acta dashboard can keep rendering the same column.
    pub fn error(&self, event: impl Into<String>, error: impl std::fmt::Display) {
        let entry = json!({ "event": event.into(), "error": error.to_string() });
        let mut fields = self.fields();
        let Some(fields) = fields.as_mut() else {
            return;
        };
        match fields.get_mut("errors") {
            Some(Value::Array(errors)) => errors.push(entry),
            _ => {
                fields.insert("errors".to_string(), Value::Array(vec![entry]));
            }
        }
    }

    /// Run a future with this event as the current one.
    ///
    /// Use this around work handed to `tokio::spawn`, which does not inherit
    /// task-local state from the task that spawned it.
    #[allow(dead_code)] // public event API for request-path code
    pub async fn scope<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        CURRENT.scope(self.clone(), future).await
    }

    /// Snapshot and close the event. Returns `None` if it was already closed.
    pub(crate) fn finish(&self) -> Option<FinishedEvent> {
        let mut fields = self.fields();
        let data = fields.take()?;
        let size = serde_json::to_vec(&data)
            .map(|json| json.len())
            .unwrap_or(0)
            + 64;
        Some(FinishedEvent {
            source: self.inner.source.clone(),
            timestamp_us: self.inner.timestamp_us,
            data,
            size,
        })
    }

    fn fields(&self) -> MutexGuard<'_, Option<Map<String, Value>>> {
        self.inner
            .fields
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

/// The event of the request this task is handling, if any.
pub fn current() -> Option<Event> {
    CURRENT.try_with(Clone::clone).ok()
}

/// Run a future with `event` as the current event.
pub async fn scope<F>(event: Event, future: F) -> F::Output
where
    F: Future,
{
    CURRENT.scope(event, future).await
}

/// Add a field to the current request's event. No-op outside a request.
#[allow(dead_code)] // public event API for request-path code
pub fn add<T: Serialize>(key: &str, value: T) {
    if let Some(event) = current() {
        event.add(key, value);
    }
}

/// Add an error to the current request's event. No-op outside a request.
pub fn error(event: impl Into<String>, error: impl std::fmt::Display) {
    if let Some(current) = current() {
        current.error(event, error);
    }
}

fn now_us() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_micros() as i64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn event_is_scoped_to_a_future() {
        let event = Event::new("test");
        event.add("method", "GET");
        assert!(current().is_none());

        let finished = event
            .scope(async {
                current().expect("current event").add("uri", "/");
                current().expect("current event").finish()
            })
            .await
            .expect("event was open");

        assert_eq!(finished.data["method"], "GET");
        assert_eq!(finished.data["uri"], "/");
        assert!(current().is_none());
        assert!(event.finish().is_none());
    }

    #[tokio::test]
    async fn finished_event_ignores_late_writes() {
        let event = Event::new("test");
        let finished = event.finish().expect("open event");
        event.add("late", true);
        assert!(finished.data.get("late").is_none());
        assert!(event.finish().is_none());
    }

    #[tokio::test]
    async fn spawned_tasks_need_an_explicit_scope() {
        let event = Event::new("test");
        event
            .scope(async {
                let inherited = tokio::spawn(async { current().is_some() }).await.unwrap();
                assert!(!inherited);

                let explicit = tokio::spawn(scope(event.clone(), async { current().is_some() }))
                    .await
                    .unwrap();
                assert!(explicit);
            })
            .await;
    }

    #[tokio::test]
    async fn concurrent_requests_do_not_mix() {
        let a = Event::new("test");
        let b = Event::new("test");

        let a_task = tokio::spawn(scope(a.clone(), async {
            tokio::task::yield_now().await;
            current().unwrap().add("who", "a");
        }));
        let b_task = tokio::spawn(scope(b.clone(), async {
            tokio::task::yield_now().await;
            current().unwrap().add("who", "b");
        }));
        a_task.await.unwrap();
        b_task.await.unwrap();

        assert_eq!(a.finish().unwrap().data["who"], "a");
        assert_eq!(b.finish().unwrap().data["who"], "b");
    }

    #[test]
    fn errors_keep_the_old_shape() {
        let event = Event::new("test");
        event.error("error", "timeout");
        event.error("Failed to read file", "denied");
        let finished = event.finish().unwrap();
        assert_eq!(
            finished.data["errors"],
            json!([
                { "event": "error", "error": "timeout" },
                { "event": "Failed to read file", "error": "denied" }
            ])
        );
    }

    #[test]
    fn merge_flattens_and_last_write_wins() {
        let event = Event::new("test");
        event.add("status", 200);
        event.merge(json!({ "status": 201, "uri": "/" }));
        let finished = event.finish().unwrap();
        assert_eq!(finished.data["status"], 201);
        assert_eq!(finished.data["uri"], "/");
    }
}
