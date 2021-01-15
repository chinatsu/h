use std::time::Duration;

use log::info;

use rdkafka::util::get_rdkafka_version;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};

mod logger;
mod environment;

use logger::setup_logger;
use environment::Environment;

async fn produce(env: &Environment) {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &env.brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let futures = (0..3)
        .map(|i| async move {
            let delivery_status = producer
                .send(
                    FutureRecord::to(&env.topic)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                    Duration::from_secs(0)
                ).await;
            info!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}


#[tokio::main]
async fn main() {
    let env = Environment::new();
    setup_logger(true, Some(&env.log_conf));

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    produce(&env).await;
}
