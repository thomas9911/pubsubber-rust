pub mod empty;
#[cfg(feature = "nats")]
pub mod nats;
#[cfg(feature = "redis")]
pub mod redis;

cfg_if::cfg_if! {
    if #[cfg(feature = "redis")] {
        pub use crate::backend::redis::{publisher, subscriber, channel};
    } else if #[cfg(feature = "nats")] {
        pub use crate::backend::nats::{publisher, subscriber, channel};
    } else {
        pub use crate::backend::empty::{publisher, subscriber, channel};
    }
}

use async_trait::async_trait;
use futures::stream::BoxStream;

#[async_trait]
pub trait PubSubBackend: PubSubSubscriberBackend + PubSubPublisherBackend {}

#[async_trait]
pub trait PubSubSubscriberBackend {
    async fn subscribe<'a>(
        &'a mut self,
        topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn unsubscribe<'a>(
        &'a mut self,
        topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    /// returns a stream that yields the messages send with the subscribed topic
    fn listen(&mut self) -> BoxStream<(String, String)>;
}

#[async_trait]
pub trait PubSubPublisherBackend {
    async fn publish<'a>(
        &'a mut self,
        topic: &str,
        message: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
