use futures::StreamExt;
use pubsubber::backend::{channel, publisher, subscriber};
use pubsubber::{PubSubPublisherBackend, PubSubSubscriberBackend};

pub async fn pubsub_test(url: &str) {
    let mut backend = subscriber(url).await.unwrap();

    backend.subscribe("pubsub_testing").await.unwrap();

    let mut publisher = publisher(url).await.unwrap();
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

pub async fn channel_test(url: &str) {
    let (mut tx, mut rx) = channel(url).await.unwrap();
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
