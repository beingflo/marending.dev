use std::any::Any;
use std::time::Instant;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use super::event::Event;
use super::sink::Sink;

/// Record one request as one wide event.
///
/// The event is created before the request is routed and closed after the
/// response headers are available. It is available both as an axum extension
/// and through the task-local `wide::current()`; the task-local is what lets
/// any code in the request path add to it without threading a handle through
/// every function signature.
pub async fn middleware(State(sink): State<Sink>, mut request: Request, next: Next) -> Response {
    let started = Instant::now();
    let event = Event::new(sink.source());
    capture_request(&event, &request);
    request.extensions_mut().insert(event.clone());

    let response = super::scope(event.clone(), next.run(request)).await;

    event.add("status", response.status().as_u16());
    event.add(
        "latency_ns",
        i64::try_from(started.elapsed().as_nanos()).unwrap_or(i64::MAX),
    );
    if response.status().is_server_error() {
        event.error("error", format!("Status code: {}", response.status()));
    }

    sink.send(event);
    response
}

/// Turn a panic in the inner service into a 500 and add the panic message to
/// the request event. Layered inside `middleware`, so the event is still in
/// scope when the panic is caught.
pub fn panic_response(error: Box<dyn Any + Send + 'static>) -> Response {
    let message = if let Some(message) = error.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = error.downcast_ref::<&str>() {
        (*message).to_string()
    } else {
        "unknown panic payload".to_string()
    };

    eprintln!("marending-dev: request panicked: {message}");
    super::error("panic", &message);

    let mut response = Response::new(Body::from("Internal Server Error"));
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
}

fn capture_request(event: &Event, request: &Request) {
    event.add("method", request.method().as_str());
    event.add("uri", request.uri().path());
    event.add("referrer", header_value(request, header::REFERER));
    event.add("user_agent", header_value(request, header::USER_AGENT));
}

fn header_value(request: &Request, name: header::HeaderName) -> &str {
    request
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{middleware, routing::get, Router};
    use tower::ServiceExt;
    use tower_http::catch_panic::CatchPanicLayer;

    use crate::wide::sink::test_sink;

    #[tokio::test]
    async fn request_events_are_isolated_and_have_the_old_shape() {
        let sink = test_sink(16);
        let app = Router::new()
            .route(
                "/a",
                get(|| async {
                    crate::wide::add("who", "a");
                    "a"
                }),
            )
            .route(
                "/b",
                get(|| async {
                    crate::wide::add("who", "b");
                    "b"
                }),
            )
            .layer(middleware::from_fn_with_state(
                sink.clone(),
                super::middleware,
            ));

        let app_a = app.clone();
        let app_b = app.clone();
        let a = tokio::spawn(async move {
            app_a
                .oneshot(Request::builder().uri("/a").body(Body::empty()).unwrap())
                .await
                .unwrap()
        });
        let b = tokio::spawn(async move {
            app_b
                .oneshot(Request::builder().uri("/b").body(Body::empty()).unwrap())
                .await
                .unwrap()
        });
        assert_eq!(a.await.unwrap().status(), StatusCode::OK);
        assert_eq!(b.await.unwrap().status(), StatusCode::OK);

        let mut events = sink.take_events();
        events.sort_by_key(|event| event.data["uri"].as_str().unwrap().to_string());
        assert_eq!(events.len(), 2);

        assert_eq!(events[0].data["uri"], "/a");
        assert_eq!(events[0].data["method"], "GET");
        assert_eq!(events[0].data["who"], "a");
        assert_eq!(events[0].data["status"].as_u64(), Some(200));
        assert!(events[0].data["latency_ns"].as_i64().unwrap() >= 0);

        assert_eq!(events[1].data["uri"], "/b");
        assert_eq!(events[1].data["who"], "b");
    }

    async fn boom() -> &'static str {
        panic!("boom")
    }

    #[tokio::test]
    async fn panics_become_a_500_event() {
        let sink = test_sink(16);
        let app = Router::new()
            .route("/panic", get(boom))
            .layer(CatchPanicLayer::custom(super::panic_response))
            .layer(middleware::from_fn_with_state(
                sink.clone(),
                super::middleware,
            ));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/panic")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let events = sink.take_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data["status"].as_u64(), Some(500));
        assert_eq!(events[0].data["errors"][0]["event"], "panic");
        assert_eq!(events[0].data["errors"][0]["error"], "boom");
    }
}
