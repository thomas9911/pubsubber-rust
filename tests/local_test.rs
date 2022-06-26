mod common;

#[tokio::test]
async fn pubsub_test() {
    common::pubsub_test("").await
}

#[tokio::test]
async fn channel_test() {
    common::channel_test("").await
}
