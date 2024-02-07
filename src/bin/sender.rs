use amqprs::{
    channel::{BasicPublishArguments, QueueDeclareArguments},
    connection::OpenConnectionArguments,
    BasicProperties,
};
use log::*;

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
        .basic_publish(
            BasicProperties::default(),
            b"hello".to_vec(),
            BasicPublishArguments {
                exchange: "".into(),
                routing_key: "hello".into(),
                ..Default::default()
            },
        )
        .await?;
    info!("Sent 'Hello world");
    connection.close().await?;
    Ok(())
}
