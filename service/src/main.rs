use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, Response},
    Router,
};
use dotenv::dotenv;
use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::signal;
use tower_http::{
    classify::ServerErrorsFailureClass,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{error, info, Span};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
use uuid::Uuid;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let tracer = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(tracer)
        .build();

    global::set_tracer_provider(provider.clone());

    // Set up tracing with both console output and OpenTelemetry
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(tracing_subscriber::filter::LevelFilter::WARN),
        )
        .with(
            OpenTelemetryLayer::new(provider.tracer("marending-service"))
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .init();

    let serve_dir = ServeDir::new("ui").not_found_service(ServeFile::new("ui/index.html"));
    let app = Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
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

    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!(message = "Starting server", port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
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
