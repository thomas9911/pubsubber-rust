use crate::backend::{PubSubPublisherBackend, PubSubSubscriberBackend};
use async_trait::async_trait;
use futures::stream::BoxStream;

pub struct EmptyBackend;

pub async fn subscriber<T>(_: T) -> Result<EmptyBackend, ()> {
    unimplemented!()
}

pub async fn publisher<T>(_: T) -> Result<EmptyBackend, ()> {
    unimplemented!()
}

pub async fn channel<T>(_: T) -> Result<(EmptyBackend, EmptyBackend), ()> {
    unimplemented!()
}

#[async_trait]
impl PubSubSubscriberBackend for EmptyBackend {
    async fn subscribe<'a>(
        &'a mut self,
        _: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        unimplemented!()
    }
    async fn unsubscribe<'a>(
        &'a mut self,
        _: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        unimplemented!()
    }

    fn listen(&mut self) -> BoxStream<(String, String)> {
        unimplemented!()
    }
}

#[async_trait]
impl PubSubPublisherBackend for EmptyBackend {
    async fn publish<'a>(
        &'a mut self,
        _: &str,
        _: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        unimplemented!()
    }
}
