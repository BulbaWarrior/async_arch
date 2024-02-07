use std::time::Duration;

use amqprs::{
    channel::{BasicConsumeArguments, Channel, QueueDeclareArguments},
    connection::OpenConnectionArguments,
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use axum::async_trait;
use log::*;
use tokio::time::sleep;
use tracing::info;

struct MyCallback;

#[async_trait]
impl AsyncConsumer for MyCallback {
    async fn consume(
        &mut self,
        _channel: &Channel,
        _deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let message = String::from_utf8(content).unwrap();
        info!("received '{message}'");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let connection = amqprs::connection::Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await?;

    let channel = connection.open_channel(None).await?;
    channel
        .queue_declare(QueueDeclareArguments::new("hello"))
        .await?;

    channel
        .basic_consume(
            MyCallback,
            BasicConsumeArguments::default()
                .queue("hello".into())
                .auto_ack(true)
                .finish(),
        )
        .await?;

    loop {
        tokio::task::yield_now().await;
    }

    // Ok(())
}
