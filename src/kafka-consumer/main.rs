use rdkafka::config::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bootstrap_servers = std::env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap_or("localhost:30001".to_string());
    let security_protocol = std::env::var("KAFKA_SECURITY_PROTOCOL").unwrap_or("PLAINTEXT".to_string());

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", "binance-consumer-group")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .set("security.protocol", &security_protocol)
        .create()
        .expect("Error creating consumer");

    println!("Subscribing to topic: binance-depth");
    consumer.subscribe(&["binance-depth"]).expect("Error subscribing to topic");

    println!("Consumer started, waiting for messages...");

    loop {
        match consumer.recv().await {
            Err(e) => eprintln!("Error receiving message: {:?}", e),
            Ok(msg) => {
                println!(
                    "Received message: topic={}, partition={}, offset={}",
                    msg.topic(),
                    msg.partition(),
                    msg.offset()
                );
                if let Some(payload) = msg.payload() {
                    let message = String::from_utf8_lossy(payload);
                    println!("Raw payload: {}", message);
                    match serde_json::from_str::<Value>(&message) {
                        Ok(json) => {
                            println!("Parsed JSON: {:?}", json);
                            let best_bid = json["bids"]
                                .as_array()
                                .and_then(|bids| bids.get(0))
                                .and_then(|bid| bid.as_array())
                                .map(|bid| bid[0].as_str().unwrap_or("N/A"));
                            let best_ask = json["asks"]
                                .as_array()
                                .and_then(|asks| asks.get(0))
                                .and_then(|ask| ask.as_array())
                                .map(|ask| ask[0].as_str().unwrap_or("N/A"));

                            println!(
                                "Processed: topic={}, partition={}, offset={}, Best Bid={}, Best Ask={}",
                                msg.topic(),
                                msg.partition(),
                                msg.offset(),
                                best_bid.unwrap_or("N/A"),
                                best_ask.unwrap_or("N/A")
                            );
                        }
                        Err(e) => eprintln!(
                            "Error parsing JSON: topic={}, partition={}, offset={}, message={:?}, error={}",
                            msg.topic(),
                            msg.partition(),
                            msg.offset(),
                            message,
                            e
                        ),
                    }
                } else {
                    eprintln!(
                        "Received empty payload: topic={}, partition={}, offset={}",
                        msg.topic(),
                        msg.partition(),
                        msg.offset()
                    );
                }
            }
        }
    }
}