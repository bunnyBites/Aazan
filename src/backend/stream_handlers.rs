use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::{self, Stream};
use std::convert::Infallible;
use tokio_stream::StreamExt as _;

pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A simple stream that sends a "ping" message every second
    let stream = stream::iter(vec!["hello", "world"])
        .map(|s| Ok(Event::default().data(s)))
        .throttle(std::time::Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
