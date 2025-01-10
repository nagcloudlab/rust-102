use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Self {
        let producer = rdkafka::config::ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Failed to create Kafka producer");

        KafkaProducer {
            producer,
            topic: topic.to_string(),
        }
    }

    pub async fn send_message(&self, key: &str, value: &str) {
        let record = FutureRecord::to(&self.topic).key(key).payload(value);

        let timeout: Timeout = Duration::from_secs(5).into(); // Explicit timeout type

        match self.producer.send(record, timeout).await {
            Ok(delivery) => println!("Message delivered: {:?}", delivery),
            Err((e, _)) => eprintln!("Failed to deliver message: {:?}", e),
        }
    }
}

impl std::fmt::Debug for KafkaProducer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KafkaProducer")
            .field("topic", &self.topic)
            .finish()
    }
}
