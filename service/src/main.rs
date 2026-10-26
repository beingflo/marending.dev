mod wide;

use std::env;

use axum::{
    http::{header::CACHE_CONTROL, HeaderValue},
    middleware, Router,
};
use dotenv::dotenv;
use tokio::signal;
use tower_http::{
    catch_panic::CatchPanicLayer,
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let Ok(port) = env::var("SERVE_PORT") else {
        eprintln!("SERVE_PORT not present in environment");
        std::process::exit(1);
    };
    let Ok(port) = port.parse::<u16>() else {
        eprintln!("SERVE_PORT is not a valid port: {port}");
        std::process::exit(1);
    };

    let exporter = match wide::Exporter::from_env() {
        Ok(exporter) => exporter,
        Err(error) => {
            eprintln!("wide: {error}");
            eprintln!("wide: request event export is disabled");
            None
        }
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

    let mut app = Router::new()
        .nest_service("/_astro", assets)
        .fallback_service(pages)
        // Inside the wide-event layer: a panic is turned into a 500 while the
        // request's event is still in scope, so the panic message can be added
        // to it before the response is recorded.
        .layer(CatchPanicLayer::custom(wide::panic_response));

    if let Some(exporter) = &exporter {
        app = app.layer(middleware::from_fn_with_state(
            exporter.sink(),
            wide::middleware,
        ));
    }

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    eprintln!("marending-dev: listening on 0.0.0.0:{port}");
    let served = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;

    // Flush even if the server itself stopped with an error.
    if let Some(exporter) = exporter {
        exporter.shutdown().await;
    }
    served?;

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

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            eprintln!("marending-dev: Ctrl+C received, shutting down")
        },
        _ = terminate => {
            eprintln!("marending-dev: SIGTERM received, shutting down")
        },
    }
}
