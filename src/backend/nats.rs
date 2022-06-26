use crate::{PubSubPublisherBackend, PubSubSubscriberBackend};
use async_nats::Client;
use async_trait::async_trait;
use futures::stream::{empty, BoxStream};
use futures::StreamExt;

pub use async_nats;

/// helper function to create a subscriber
/// you can create nats client manually because subscriber is implemented on `async_nats::Client`
pub async fn subscriber<T: async_nats::ToServerAddrs>(
    connection_info: T,
) -> Result<NatsBackend, async_nats::Error> {
    Ok(async_nats::connect(connection_info).await?.into())
}

/// helper function to create a publisher
/// you can create nats client manually because publisher is implemented on `async_nats::Client`
pub async fn publisher<T: async_nats::ToServerAddrs>(
    connection_info: T,
) -> Result<NatsBackend, async_nats::Error> {
    Ok(async_nats::connect(connection_info).await?.into())
}

pub async fn channel<T: async_nats::ToServerAddrs + Clone>(
    connection_info: T,
) -> Result<(NatsBackend, NatsBackend), async_nats::Error> {
    Ok((
        publisher(connection_info.clone()).await?,
        subscriber(connection_info.clone()).await?,
    ))
}

pub struct NatsBackend {
    client: Client,
    subscriber: Option<async_nats::Subscriber>,
}

impl From<Client> for NatsBackend {
    fn from(client: Client) -> NatsBackend {
        NatsBackend {
            client,
            subscriber: None,
        }
    }
}

impl From<NatsBackend> for Client {
    fn from(backend: NatsBackend) -> Client {
        backend.client
    }
}

#[async_trait]
impl PubSubSubscriberBackend for NatsBackend {
    async fn subscribe<'a>(
        &'a mut self,
        topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.subscriber.is_some() {
            return Err("subscriber already set".into());
        }

        self.subscriber = Some(self.client.subscribe(topic.to_string()).await?);
        Ok(())
    }
    async fn unsubscribe<'a>(
        &'a mut self,
        _topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &mut self.subscriber {
            None => Ok(()),
            Some(subscriber) => Ok(subscriber.unsubscribe().await?),
        }
    }

    fn listen(&mut self) -> BoxStream<(String, String)> {
        match &mut self.subscriber {
            None => Box::pin(empty()),
            Some(subscriber) => Box::pin(subscriber.map(|msg| {
                (
                    msg.subject,
                    String::from_utf8((&msg.payload).to_vec()).unwrap_or(String::new()),
                )
            })),
        }
    }
}

#[async_trait]
impl PubSubPublisherBackend for NatsBackend {
    async fn publish<'a>(
        &'a mut self,
        topic: &str,
        message: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.client
            .publish(topic.to_owned(), message.into())
            .await?;
        Ok(())
    }
}
