use std::time::{Duration, Instant};

use log::{info, debug, error};

use rdkafka::util::get_rdkafka_version;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};

mod logger;
mod environment;

use logger::setup_logger;
use environment::Environment;

async fn produce(producer: &FutureProducer, env: &Environment) {
    for i in 0..env.messages {
        let payload = format!("Message {}", i);
        let key = format!("Key {}", i);
        async move {
            let delivery_status = producer.send(
                    FutureRecord::to(&env.topic)
                        .payload(&payload)
                        .key(&key)
                        .headers(OwnedHeaders::new().add("header_key", "header_value")),
                    Duration::from_secs(0)
                );
            match delivery_status.await {
                Ok(delivery) => debug!("Sent: {:?}", delivery),
                Err((e, _)) => error!("{:?}", e)
            }
        }.await;
    }
}


#[tokio::main]
async fn main() {
    let env = Environment::new();
    setup_logger(true, Some(&env.log_conf));

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &env.brokers)
        .set("message.timeout.ms", "15000")
        .set("enable.idempotence", "true")
        .set("retries", "100000")
        .set("max.in.flight", "1")
        .set("queue.buffering.max.ms", "0")
        .set("batch.num.messages", "10000")
        .set("queue.buffering.backpressure.threshold", "1")
        .set("acks", "-1")
        .create()
        .expect("Producer creation error");

    info!("Sending {} messages", env.messages);
    let start = Instant::now();
    produce(producer, &env).await;
    let duration = start.elapsed();
    info!("Time elapsed in produce() is: {:?}", duration);
}
