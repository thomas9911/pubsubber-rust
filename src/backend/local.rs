use crate::{PubSubPublisherBackend, PubSubSubscriberBackend};
use async_trait::async_trait;
use circulate::{Relay, Subscriber};
use futures::stream::BoxStream;
use futures::StreamExt;
use once_cell::sync::OnceCell;

pub use circulate;

static RELAY: OnceCell<Relay> = OnceCell::new();

pub struct CirculateBackend(Relay);

/// helper function to create a subscriber
pub async fn subscriber(_: &str) -> Result<Subscriber, ()> {
    let relay = RELAY.get_or_init(Relay::default);
    Ok(relay.create_subscriber())
}

/// helper function to create a publisher
pub async fn publisher(_: &str) -> Result<CirculateBackend, ()> {
    Ok(CirculateBackend(RELAY.get_or_init(Relay::default).clone()))
}

pub async fn channel(connection_info: &str) -> Result<(CirculateBackend, Subscriber), ()> {
    Ok((
        publisher(connection_info.clone()).await?,
        subscriber(connection_info.clone()).await?,
    ))
}

#[async_trait]
impl PubSubSubscriberBackend for Subscriber {
    async fn subscribe<'a>(
        &'a mut self,
        topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.subscribe_to(&topic)?;
        Ok(())
    }
    async fn unsubscribe<'a>(
        &'a mut self,
        topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.unsubscribe_from(&topic)?;
        Ok(())
    }

    fn listen(&mut self) -> BoxStream<(String, String)> {
        Box::pin(self.receiver().stream().map(|msg| {
            (
                msg.topic().unwrap_or(String::new()),
                msg.payload().unwrap_or(String::new()),
            )
        }))
    }
}

#[async_trait]
impl PubSubPublisherBackend for CirculateBackend {
    async fn publish<'a>(
        &'a mut self,
        topic: &str,
        message: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.0.publish(&topic, &message)?;
        Ok(())
    }
}
