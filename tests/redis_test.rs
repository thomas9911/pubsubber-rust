use futures::StreamExt;
use pubsubber::backend::redis::{channel, publisher, subscriber};
use pubsubber::backend::{PubSubPublisherBackend, PubSubSubscriberBackend};

#[tokio::test]
async fn pubsub_test() {
    let redis_url = "redis://127.0.0.1/";

    let mut backend = subscriber(redis_url).await.unwrap();

    backend.subscribe("pubsub_testing").await.unwrap();

    let mut publisher = publisher(redis_url).await.unwrap();
    tokio::spawn(async move {
        publisher
            .publish("pubsub_testing", String::from("test data one"))
            .await
            .unwrap();

        publisher
            .publish("pubsub_testing", String::from("test data two"))
            .await
            .unwrap();
    });

    let mut stream = backend.listen();
    let a = stream.next().await;
    let b = stream.next().await;

    assert_eq!(
        Some((
            String::from("pubsub_testing"),
            String::from("test data one")
        )),
        a
    );
    assert_eq!(
        Some((
            String::from("pubsub_testing"),
            String::from("test data two")
        )),
        b
    );
}

#[tokio::test]
async fn channel_test() {
    let redis_url = "redis://127.0.0.1/";

    let (mut tx, mut rx) = channel(redis_url).await.unwrap();
    rx.subscribe("channel_testing").await.unwrap();

    tokio::spawn(async move {
        tx.publish("channel_testing", String::from("test data one"))
            .await
            .unwrap();

        tx.publish("channel_testing", String::from("test data two"))
            .await
            .unwrap();
    });

    let mut stream = rx.listen();
    let a = stream.next().await;
    let b = stream.next().await;

    assert_eq!(
        Some((
            String::from("channel_testing"),
            String::from("test data one")
        )),
        a
    );
    assert_eq!(
        Some((
            String::from("channel_testing"),
            String::from("test data two")
        )),
        b
    );
}
