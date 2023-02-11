use std::env;

use rdkafka::Message;
use tokio::runtime::Builder;

use crate::kafka::consumer::LoggingConsumer;

pub mod kafka {
    pub mod consumer;
    pub mod source;
}

pub mod mapper {
    pub mod json_to_avro_mapper;
}

pub mod config {
    pub mod conf;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::conf::load_config(args.get(2).get_or_insert(&"".to_string())).unwrap();
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let consumer: LoggingConsumer = kafka::consumer::create(&config["kafka-clients.consumer"]);
            loop {
                println!("looping");
                match consumer.recv().await {
                    Err(e) => {
                        println!("Kafka error: {}", e);
                    }
                    Ok(m) => {
                        println!("here{:?}", m.payload());
                    }
                }
            };
        })
}

