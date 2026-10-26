//! Wide events: one JSON object per request, buffered and sent to Acta.
//!
//! # Async context
//!
//! The current event lives in a [`tokio::task_local`], not a thread-local.
//! `middleware` runs the rest of the request inside that scope, so the event is
//! bound for every poll of the request future and restored when the future
//! yields. Two requests sharing a worker thread therefore never see each
//! other's event.
//!
//! `tokio::spawn` starts a new task with fresh task-local storage, exactly like
//! a new thread would. Code that needs the request event from a spawned task
//! must opt in and carry the handle along:
//!
//! ```ignore
//! let event = wide::current();
//! tokio::spawn(async move {
//!     if let Some(event) = event {
//!         event.scope(async { wide::add("background", true) }).await;
//!     }
//! });
//! ```
//!
//! This is the same model `tracing` uses for spans: the context follows the
//! instrumented future, not the thread.
//!
//! The event is closed when the response headers are ready. Work that has to be
//! part of the event must therefore finish before the handler returns; a
//! detached task that writes later is ignored.

mod event;
mod request;
mod sink;

#[allow(unused_imports)] // the event handle API is for request-path code
pub use event::{add, current, error, scope, Event};
pub use request::{middleware, panic_response};
pub use sink::Exporter;
