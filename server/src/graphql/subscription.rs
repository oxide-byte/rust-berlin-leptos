use std::time::Duration;
use chrono::Utc;
use futures::stream::{BoxStream, StreamExt as _};
use juniper::{FieldError, graphql_subscription};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tracing::log::{Level, log};
use crate::graphql::{ClockBox, ServerContext};

#[derive(Clone, Debug)]
pub struct Subscription {}

type StringStream = BoxStream<'static, Result<ClockBox, FieldError>>;

#[graphql_subscription(context = ServerContext)]
impl Subscription {

    async fn clock(
        #[graphql(context)] _server_context: &ServerContext
    ) -> StringStream {
        log!(Level::Info, "Subscription to clock...");
        
        let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
            Ok(ClockBox { clock: format!("{}", Utc::now().to_string()) } )
        });
        
        Box::pin(stream)
    }
}