use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    // Configure the producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    // Send a message
    let topic = "greetings";
    let value = "Hello, Kafka!";

    let delivery_status = producer
        .send(
            FutureRecord::to(topic).payload(value).key("key1"),
            Duration::from_secs(0),
        )
        .await;

    // Check the result of the delivery
    match delivery_status {
        Ok(delivery) => println!("Message delivered: {:?}", delivery),
        Err((e, _message)) => eprintln!("Delivery failed: {:?}", e),
    }
}
