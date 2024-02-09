use futures::StreamExt;
use log::*;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9094")
        .set("group.id", "0")
        .create()?;

    info!("created kafka consumer");
    consumer.subscribe(&["hello"])?;

    info!("subscribed to the topic");
    consumer
        .stream()
        .for_each(|res| async {
            info!("processing message");
            let msg = match res {
                Err(err) => {
                    error!("error getting message from topic: {err}");
                    return;
                }
                Ok(x) => x,
            };
            let body = String::from_utf8(msg.payload().unwrap().to_vec()).unwrap();
            info!("received message '{body}'");
        })
        .await;

    Ok(())
}
