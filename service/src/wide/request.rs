use std::any::Any;
use std::time::Instant;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode, Uri},
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
    let route_class = classify_path(request.uri().path());
    capture_request(&event, &request);
    request.extensions_mut().insert(event.clone());

    let response = super::scope(event.clone(), next.run(request)).await;

    capture_response(&event, route_class, &response);
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
    let headers = request.headers();
    event.add("method", request.method().as_str());
    event.add("uri", request.uri().path());
    event.add("host", host(headers));
    event.add("referrer", header_value(headers, header::REFERER));
    event.add("referrer_host", referrer_host(headers));
    event.add("user_agent", header_value(headers, header::USER_AGENT));
    event.add("accept_language", accept_language(headers));
    // Whether the client offered a validator, so `status: 304` can be read as a
    // cache revalidation rather than a surprise. The 304 rate itself is a query
    // over `status`, not a field.
    event.add("conditional_request", is_conditional(headers));
}

fn capture_response(event: &Event, route_class: &'static str, response: &Response) {
    let status = response.status();
    let headers = response.headers();
    event.add("status", status.as_u16());
    event.add("route_class", class_for_status(route_class, status));
    event.add("content_type", header_value(headers, header::CONTENT_TYPE));
    // Absent means the body went out uncompressed; the header is only set when
    // ServeDir found a `.br`/`.gz` sibling, so this is what proves the
    // precompressed assets are actually being used.
    event.add(
        "content_encoding",
        header_value(headers, header::CONTENT_ENCODING),
    );
    if let Ok(length) = header_value(headers, header::CONTENT_LENGTH).parse::<i64>() {
        event.add("content_length", length);
    }
}

/// Coarse class of a request path. The router has two branches: `/_astro` is the
/// immutable hashed asset tree, everything else is served out of `ui/` by the
/// page service, which also serves images and feeds.
fn classify_path(path: &str) -> &'static str {
    if path == "/_astro" || path.starts_with("/_astro/") {
        return "asset";
    }
    // The last segment is empty for a directory request (`/notes/foo/`, `/`).
    let segment = path.rsplit('/').next().unwrap_or_default();
    match segment.rsplit_once('.') {
        None => "page",
        Some((_, "html" | "htm")) => "page",
        Some(_) => "file",
    }
}

/// `missing` wins over the path class: a 404 means the file was not there and
/// `ui/404.html` was served instead, whatever the path looked like.
fn class_for_status(route_class: &'static str, status: StatusCode) -> &'static str {
    if status == StatusCode::NOT_FOUND {
        "missing"
    } else {
        route_class
    }
}

/// The host the request was addressed to, without the port.
fn host(headers: &HeaderMap) -> String {
    let host = header_value(headers, header::HOST);
    let host = match host.rsplit_once(':') {
        // Never strip when the head still has a colon: that is a bare IPv6
        // literal, not `name:port`.
        Some((head, port))
            if !head.contains(':')
                && !port.is_empty()
                && port.chars().all(|c| c.is_ascii_digit()) =>
        {
            head
        }
        _ => host,
    };
    host.to_ascii_lowercase()
}

/// The origin of the `Referer` header, lowercased; empty when absent or not a
/// URL. Browsers send only the origin cross-site (`strict-origin-when-cross-origin`).
fn referrer_host(headers: &HeaderMap) -> String {
    header_value(headers, header::REFERER)
        .parse::<Uri>()
        .ok()
        .and_then(|uri| uri.host().map(str::to_ascii_lowercase))
        .unwrap_or_default()
}

/// The first language tag of `Accept-Language`, without its weight, e.g. `de-ch`.
fn accept_language(headers: &HeaderMap) -> String {
    header_value(headers, header::ACCEPT_LANGUAGE)
        .split(',')
        .next()
        .unwrap_or_default()
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
}

fn is_conditional(headers: &HeaderMap) -> bool {
    headers.contains_key(header::IF_NONE_MATCH) || headers.contains_key(header::IF_MODIFIED_SINCE)
}

fn header_value(headers: &HeaderMap, name: header::HeaderName) -> &str {
    headers
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
                .oneshot(
                    Request::builder()
                        .uri("/a")
                        .header(header::HOST, "Marending.dev:3004")
                        .header(header::ACCEPT_LANGUAGE, "de-CH,de;q=0.9,en;q=0.8")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap()
        });
        let b = tokio::spawn(async move {
            app_b
                .oneshot(
                    Request::builder()
                        .uri("/b")
                        .header(header::IF_NONE_MATCH, "\"v1\"")
                        .body(Body::empty())
                        .unwrap(),
                )
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
        assert_eq!(events[0].data["host"], "marending.dev");
        assert_eq!(events[0].data["accept_language"], "de-ch");
        assert_eq!(events[0].data["route_class"], "page");
        assert_eq!(events[0].data["conditional_request"], false);

        assert_eq!(events[1].data["uri"], "/b");
        assert_eq!(events[1].data["who"], "b");
        assert_eq!(events[1].data["host"], "");
        assert_eq!(events[1].data["route_class"], "page");
        assert_eq!(events[1].data["conditional_request"], true);
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

    fn headers(pairs: &[(header::HeaderName, &str)]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for (name, value) in pairs {
            headers.append(name.clone(), value.parse().unwrap());
        }
        headers
    }

    #[test]
    fn paths_are_classified_by_router_branch() {
        assert_eq!(classify_path("/"), "page");
        assert_eq!(classify_path("/about/"), "page");
        assert_eq!(classify_path("/notes/wide-events/"), "page");
        assert_eq!(classify_path("/404.html"), "page");
        assert_eq!(classify_path("/_astro/index.abc123.js"), "asset");
        assert_eq!(classify_path("/notes/wide-events/charts.jpg"), "file");
        assert_eq!(classify_path("/rss.xml"), "file");
        assert_eq!(classify_path("/robots.txt"), "file");
        // A dot in a directory name is not an extension.
        assert_eq!(classify_path("/notes/1.0/"), "page");
    }

    #[test]
    fn not_found_overrides_the_path_class() {
        assert_eq!(class_for_status("page", StatusCode::NOT_FOUND), "missing");
        assert_eq!(class_for_status("asset", StatusCode::OK), "asset");
        assert_eq!(
            class_for_status("file", StatusCode::INTERNAL_SERVER_ERROR),
            "file"
        );
    }

    #[test]
    fn host_drops_the_port_and_is_lowercased() {
        assert_eq!(
            host(&headers(&[(header::HOST, "Marending.dev:3004")])),
            "marending.dev"
        );
        assert_eq!(
            host(&headers(&[(header::HOST, "marending.dev")])),
            "marending.dev"
        );
        assert_eq!(host(&headers(&[])), "");
    }

    #[test]
    fn referrer_is_reduced_to_its_host() {
        assert_eq!(
            referrer_host(&headers(&[(
                header::REFERER,
                "https://News.YCombinator.com/item?id=1"
            )])),
            "news.ycombinator.com"
        );
        assert_eq!(referrer_host(&headers(&[(header::REFERER, "")])), "");
        assert_eq!(
            referrer_host(&headers(&[(header::REFERER, "not a url")])),
            ""
        );
    }

    #[test]
    fn accept_language_keeps_the_first_tag() {
        assert_eq!(
            accept_language(&headers(&[(header::ACCEPT_LANGUAGE, "de-CH,de;q=0.9")])),
            "de-ch"
        );
        assert_eq!(
            accept_language(&headers(&[(header::ACCEPT_LANGUAGE, "*;q=0.1")])),
            "*"
        );
        assert_eq!(accept_language(&headers(&[])), "");
    }
}
