# Pubsubber

interface for multiple pubsub backends.

Current backends:

- Nats
- Redis
- Local, without external service (implemented with Circulate)

## Getting started

```toml
pubsubber = { version = "*", features = ["redis"] }
```

```toml
pubsubber = { version = "*", features = ["nats"] }
```

```toml
pubsubber = { version = "*", features = ["local"] }
```

## Examples

```rust
use pubsubber::StreamExt;
use pubsubber::{channel, publisher, subscriber};
use pubsubber::{PubSubPublisherBackend, PubSubSubscriberBackend};

pub async fn pubsub_test(url: &str) {
    let mut backend = subscriber(url).await.unwrap();

    backend.subscribe("testing").await.unwrap();

    let mut publisher = publisher(url).await.unwrap();
    tokio::spawn(async move {
        publisher
            .publish("testing", String::from("test data one"))
            .await
            .unwrap();
    });

    // returns a stream that yields a tuple (channel, message): (String, String)
    let mut stream = backend.listen();
}
```

## Note

This is a simple interface which makes it easy to swap between backends, however there are some choices I made which will not be appropriate for your use case. For instance:

- All messages that are send are `String`s (or utf8 bytes)
- you can only subscribe to a specific channel (so no pattern channels (psubscribe in Redis))
