use crate::backend::{PubSubPublisherBackend, PubSubSubscriberBackend};
use async_trait::async_trait;
use futures::stream::BoxStream;
use futures::StreamExt;

use redis::AsyncCommands;
use redis::Value;

/// helper function to create a subscriber
pub async fn subscriber<T: redis::IntoConnectionInfo>(
    connection_info: T,
) -> redis::RedisResult<redis::aio::PubSub> {
    let client = redis::Client::open(connection_info)?;
    let con = client.get_async_connection().await?;
    Ok(con.into_pubsub())
}

/// helper function to create a publisher
/// publisher is also implemented for `redis::aio::ConnectionLike` so you can use your own connection
pub async fn publisher<T: redis::IntoConnectionInfo>(
    connection_info: T,
) -> redis::RedisResult<redis::aio::Connection> {
    let client = redis::Client::open(connection_info)?;
    let con = client.get_async_connection().await?;
    Ok(con)
}

pub async fn channel<T: redis::IntoConnectionInfo + Clone>(
    connection_info: T,
) -> redis::RedisResult<(redis::aio::Connection, redis::aio::PubSub)> {
    Ok((
        publisher(connection_info.clone()).await?,
        subscriber(connection_info.clone()).await?,
    ))
}

#[async_trait]
impl PubSubSubscriberBackend for redis::aio::PubSub {
    async fn subscribe<'a>(
        &'a mut self,
        topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.subscribe(topic).await?;
        Ok(())
    }
    async fn unsubscribe<'a>(
        &'a mut self,
        topic: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.unsubscribe(topic).await?;
        Ok(())
    }

    fn listen(&mut self) -> BoxStream<(String, String)> {
        Box::pin(self.on_message().map(|msg| {
            (
                msg.get_channel_name().to_string(),
                msg.get_payload::<Value>().map(value_to_string).unwrap(),
            )
        }))
    }
}

#[async_trait]
impl<C: redis::aio::ConnectionLike + Sync + Send + 'static> PubSubPublisherBackend for C {
    async fn publish<'a>(
        &'a mut self,
        topic: &str,
        message: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        AsyncCommands::publish(self, topic, message).await?;
        Ok(())
    }
}

fn value_to_string(value: Value) -> String {
    inner_value_to_string(value).unwrap_or(String::new())
}

fn inner_value_to_string(value: Value) -> redis::RedisResult<String> {
    match value {
        Value::Data(ref bytes) => Ok(std::str::from_utf8(bytes)?.to_string()),
        Value::Okay => Ok("OK".to_string()),
        Value::Status(ref val) => Ok(val.to_string()),
        Value::Int(int) => Ok(int.to_string()),
        _ => Err((redis::ErrorKind::TypeError, "").into()),
    }
}
