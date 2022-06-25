#[cfg(feature = "nats")]
pub mod nats;
#[cfg(feature = "redis")]
pub mod redis;

use async_trait::async_trait;
use futures::stream::BoxStream;

#[async_trait]
pub trait PubSubBackend: PubSubSubscriberBackend + PubSubPublisherBackend {}

#[async_trait]
pub trait PubSubSubscriberBackend {
    async fn subscribe<'a>(&'a mut self, topic: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn unsubscribe<'a>(&'a mut self, topic: &str) -> Result<(), Box<dyn std::error::Error>>;
    /// returns a stream that yields the messages send with the subscribed topic
    fn listen(&mut self) -> BoxStream<(String, String)>;
}

#[async_trait]
pub trait PubSubPublisherBackend {
    async fn publish<'a>(
        &'a mut self,
        topic: &str,
        message: String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
