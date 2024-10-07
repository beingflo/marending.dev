use std::time::Duration;

use axum::{
    body::Body,
    http::{Request, Response},
    Router,
};
use tokio::signal;
use tower_http::{
    classify::ServerErrorsFailureClass,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{info, Span};
use tracing_subscriber::fmt::format::FmtSpan;
use uuid::Uuid;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
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
                    tracing::info!("request: {} {}", request.method(), request.uri().path())
                })
                .on_response(
                    |response: &Response<Body>, latency: Duration, _span: &Span| {
                        tracing::info!("response: {} {:?}", response.status(), latency)
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("error: {}", error)
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
