use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bootstrap_servers = std::env::var("KAFKA_BOOTSTRAP_SERVERS")
        .unwrap_or("localhost:30001".to_string());
    let security_protocol = std::env::var("KAFKA_SECURITY_PROTOCOL")
        .unwrap_or("PLAINTEXT".to_string());
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("enable.idempotence", "true")
        .set("security.protocol", &security_protocol)
        .create()
        .expect("Error producer");

    let client = Client::new();
    let url = "https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000";

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        let response = match client.get(url).send().await {
            Ok(resp) => match resp.json::<Value>().await {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("Error parsing Binance response: {:?}", e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("Error fetching Binance data: {:?}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        let payload = match serde_json::to_string(&response) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error serializing payload: {:?}", e);
                continue;
            }
        };

        let topic = "binance-depth";
        let record = FutureRecord::to(topic)
            .payload(&payload)
            .key("BTCUSDT");

        match producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => println!("Data sent to Kafka: {}", payload),
            Err((e, _)) => eprintln!("Error sending data to Kafka: {:?}", e),
        }
    }
}