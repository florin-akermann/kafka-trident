use std::collections::HashMap;
use std::env;
use std::io::Cursor;

use rdkafka::Message;
use tokio::runtime::Builder;

use murmur3::murmur3_x64_128;

use crate::kafka::consumer::LoggingConsumer;

pub mod kafka {
    pub mod consumer;
    pub mod source;
}

pub mod config {
    pub mod conf;
}

fn main() {
    let mut state = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let config = config::conf::load_config(args.get(2).get_or_insert(&"".to_string())).unwrap();
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let consumer: LoggingConsumer = kafka::consumer::create(&config["kafka-clients.consumer"]);
            loop {
                match consumer.recv().await {
                    Err(e) => {
                        println!("Kafka error: {}", e);
                    }
                    Ok(m) => {
                        let value = m.payload().unwrap();
                        let hashed_value = murmur3_x64_128(&mut Cursor::new(value), 0);
                        let v = hashed_value.unwrap();
                        state.insert(v, v);
                    }
                }
            };
        })
}

