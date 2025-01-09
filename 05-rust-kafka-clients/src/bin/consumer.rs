use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::Message;
use tokio;

#[tokio::main]
async fn main() -> Result<(), KafkaError> {
    // Configure the Kafka consumer
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "g1")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Consumer creation failed");

    // Subscribe to topics
    consumer
        .subscribe(&["greetings"])
        .expect("Failed to subscribe to topic");

    println!("Starting Kafka consumer...");

    // Consume messages asynchronously
    loop {
        match consumer.recv().await {
            Ok(message) => {
                // Process the received message
                if let Some(payload) = message.payload() {
                    // Print Topic , partition , offset and payload
                    println!(
                        "Received message: {} from topic: {}",
                        String::from_utf8_lossy(payload),
                        message.topic()
                    );
                }

                if let Some(key) = message.key() {
                    println!("Key: {}", String::from_utf8_lossy(key));
                }
            }
            Err(err) => {
                eprintln!("Error while consuming messages: {:?}", err);
            }
        }
    }
}
