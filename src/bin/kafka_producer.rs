use std::time::Duration;

use futures::{stream::FuturesUnordered, StreamExt};
use log::*;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "127.0.0.1:9094")
        .set("message.timeout.ms", "5000")
        .create()?;

    let producer = &producer;

    let futures: FuturesUnordered<_> = (1..5)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to("hello")
                        .payload(&format!("hello there {i}"))
                        .key(&format!("{i}")),
                    Duration::from_secs(0),
                )
                .await;
            tokio::time::sleep(Duration::from_secs(3)).await;
            info!("Delivery status for message {i} received");
            delivery_status
        })
        .collect();

    let _res = futures
        .map(|v| info!("result: {v:?}"))
        .collect::<Vec<_>>()
        .await;

    Ok(())
}
