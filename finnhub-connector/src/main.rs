use clap::Parser;
use dotenv::dotenv;
use rdkafka::producer::{FutureProducer, FutureRecord};
use reqwest_websocket::RequestBuilderExt;
// use futures_util::SinkExt;
use futures_util::{SinkExt, StreamExt as _};
use serde::{Deserialize, Serialize};
#[derive(Parser)]
#[command(long_about=None)]
struct Args {
    #[arg(long, default_value_t = String::from("BINANCE:BTCUSDT"))]
    symbol: String,
    #[arg(long, default_value_t = String::from("binance_usdt_topic"))]
    topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckMessage {
    r#type: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let (tx, mut rx) = tokio::sync::mpsc::channel(64);

    let args = Args::parse();
    let ws_url = format!(
        "wss://ws.finnhub.io?token={}",
        std::env::var("TOKEN").unwrap()
    );
    let symbol = format!(
        "{{\"type\":\"subscribe\", \"symbol\": \"{}\" }}",
        args.symbol
    );
    let brokers = std::env::var("KAFKA_BROKER").unwrap();
    let topic = args.topic;

    let response = reqwest::Client::default()
        .get(ws_url)
        .upgrade()
        .send()
        .await
        .unwrap();

    let mut websocket = response.into_websocket().await.unwrap();

    let producer: &FutureProducer = &rdkafka::ClientConfig::new()
        .set("group.id", "finnhub-connector")
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Failed create producer");

    tokio::spawn(async move {
        websocket
            .send(reqwest_websocket::Message::Text(symbol.into()))
            .await
            .unwrap();

        while let Some(message) = websocket.next().await {
            if let reqwest_websocket::Message::Text(text) = message.unwrap() {
                // println!("{}", text);
                tx.send(text).await.unwrap()
            }
        }
    });

    while let Some(message) = rx.recv().await {
        let type_message: CheckMessage = serde_json::from_str(&message).unwrap();
        if type_message.r#type.eq("trade") {
            producer
                .send(
                    FutureRecord::to(&topic)
                        .key("some key")
                        .payload(message.as_str()),
                    std::time::Duration::from_secs(0),
                )
                .await
                .unwrap();
        } else if type_message.r#type.eq("ping") {
            println!("waiting data ...")
        } else {
            println!("the type not defined")
        }
    }
}
