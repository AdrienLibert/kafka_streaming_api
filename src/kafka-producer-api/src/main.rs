use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:30001")
        .create()
        .expect("Error producer");

    let client = Client::new();
    let url = "https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000";

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;

        let response = client.get(url).send().await?.json::<Value>().await?;
        
        let payload = serde_json::to_string(&response)?;

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