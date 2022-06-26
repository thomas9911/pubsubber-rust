mod common;

#[tokio::test]
async fn pubsub_test() {
    let nats_url = "127.0.0.1";

    common::pubsub_test(nats_url).await
}

#[tokio::test]
async fn channel_test() {
    let nats_url = "127.0.0.1";

    common::channel_test(nats_url).await
}
