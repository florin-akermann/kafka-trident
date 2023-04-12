use std::collections::HashMap;
use std::convert::TryInto;

use hocon::Hocon;
use rdkafka::{ClientConfig, ClientContext, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, ConsumerContext, StreamConsumer};
use rdkafka::error::KafkaResult;

pub struct LoggingConsumerContext;

impl ClientContext for LoggingConsumerContext {}

impl ConsumerContext for LoggingConsumerContext {
    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        match result {
            Ok(_) => println!("Offsets committed successfully"),
            Err(e) => println!("Error while committing offsets: {}", e),
        };
    }
}

pub type LoggingConsumer = StreamConsumer<LoggingConsumerContext>;

pub fn create(config: &Hocon) -> LoggingConsumer {
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("group.id", "some-group")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(LoggingConsumerContext)
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["test-topic2"])
        .expect("Can't subscribe to specified topic");
    consumer
}

