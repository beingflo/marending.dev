use std::{env, time::Duration};

use axum::{
    body::Body,
    http::{header::CACHE_CONTROL, HeaderValue, Request, Response},
    Router,
};
use dotenv::dotenv;
use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::signal;
use tower_http::{
    classify::ServerErrorsFailureClass,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing::{error, info, Span};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
use uuid::Uuid;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let provider = match init_telemetry() {
        Ok(provider) => Some(provider),
        Err(error) => {
            eprintln!("OpenTelemetry unavailable, continuing without it: {error}");
            None
        }
    };

    // Set up tracing with both console output and OpenTelemetry
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(tracing_subscriber::filter::LevelFilter::WARN),
        )
        .with(provider.as_ref().map(|provider| {
            OpenTelemetryLayer::new(provider.tracer("marending-service"))
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO)
        }))
        .init();

    let Ok(port) = env::var("SERVE_PORT") else {
        error!("SERVE_PORT not present in environment");
        panic!()
    };
    let Ok(port) = port.parse::<u16>() else {
        error!(message = "SERVE_PORT is not a valid port", port);
        panic!()
    };

    // Hashed asset names never change contents, everything else must revalidate.
    let assets = Router::new()
        .fallback_service(
            ServeDir::new("ui/_astro")
                .precompressed_gzip()
                .precompressed_br(),
        )
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        ));
    let pages = Router::new()
        .fallback_service(
            ServeDir::new("ui")
                .precompressed_gzip()
                .precompressed_br()
                .not_found_service(ServeFile::new("ui/404.html")),
        )
        .layer(SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=0, must-revalidate"),
        ));

    let app = Router::new()
        .nest_service("/_astro", assets)
        .fallback_service(pages)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| {
                    let request_id = Uuid::new_v4().to_string();
                    tracing::info_span!("http-request", %request_id)
                })
                .on_request(|request: &Request<Body>, _span: &Span| {
                    info!(
                        message = "request",
                        request = request.method().as_str(),
                        uri = request.uri().path().to_string(),
                        referrer = request
                            .headers()
                            .get("referer")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or(""),
                        user_agent = request
                            .headers()
                            .get("user-agent")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("")
                    )
                })
                .on_response(
                    |response: &Response<Body>, latency: Duration, _span: &Span| {
                        info!(
                            message = "response_status",
                            status = response.status().as_u16(),
                            latency = latency.as_nanos()
                        )
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        error!(message = "error", error = error.to_string())
                    },
                ),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!(message = "Starting server", port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    if let Some(provider) = provider {
        provider.shutdown()?;
    }

    Ok(())
}

fn init_telemetry() -> Result<SdkTracerProvider, Box<dyn std::error::Error>> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .build();

    global::set_tracer_provider(provider.clone());

    Ok(provider)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            info!("Ctrl+C received, shutting down")
        },
        _ = terminate => {
            info!("SIGTERM received, shutting down")
        },
    }
}
