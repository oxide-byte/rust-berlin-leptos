use crate::graphql::ClockBox;
use chrono::Utc;
use async_graphql::{Context, Subscription};
use tokio_stream::StreamExt as _;
use std::time::Duration;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tracing::log::{log, Level};

#[derive(Clone, Debug)]
pub struct Subscription {}

#[Subscription]
impl Subscription {
    async fn clock(&self, _ctx: &Context<'_>) -> impl tokio_stream::Stream<Item = ClockBox> {
        log!(Level::Info, "Subscription to clock...");

        IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
            ClockBox { clock: format!("{}", Utc::now().to_string()) }
        })
    }
}