mod event_handler;

use std::error::Error;
use std::fs::File;
use std::ops::Deref;
use std::path::PathBuf;
use dotenv::dotenv;
use iota_client::{Client, MqttEvent, MqttPayload, Result, Topic};
use std::sync::{mpsc::channel, Arc, Mutex};
use std::{io, thread};
use std::io::Read;
use std::time::Duration;
use async_std::task;
use flate2::read::ZlibDecoder;
use iota_client::block::Block;
use iota_client::block::payload::Payload;
use log::LevelFilter::Debug;
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let logger_output_config = fern_logger::LoggerOutputConfigBuilder::new()
        .name("indexer.log")
        .target_exclusions(&["h2", "hyper", "rustls","iota_wallet","iota_client","reqwest","tungstenite","rumqttc"])
        .level_filter(Debug);

    let config = fern_logger::LoggerConfig::build()
        .with_output(logger_output_config)
        .finish();
    fern_logger::logger_init(config).unwrap();

    let connection = Connection::open(std::env::var("DATABASE_PATH").unwrap()).unwrap();
    connection.busy_timeout(Duration::from_secs(5)).unwrap();
    let script = std::fs::read_to_string("createTables.sql").unwrap();
    connection.execute_batch(&script).unwrap();

    let client = Client::builder()
        .with_node(std::env::var("NODE_URL").unwrap().as_str()).unwrap()
        .with_pow_worker_count(std::env::var("NUM_OF_WORKERS").unwrap().parse().unwrap())
        .with_local_pow(true)
        .finish().unwrap();

    let (tx, rx) = channel();
    let tx = Arc::new(Mutex::new(tx));

    let mut event_rx = client.mqtt_event_receiver();
    tokio::spawn(async move {
        while event_rx.changed().await.is_ok() {
            let event = event_rx.borrow();
            if *event == MqttEvent::Disconnected {
                println!("mqtt disconnected");
                std::process::exit(1);
            }
        }
    });

    let tag = hex::encode("EDCAS");

    let topic = format!("blocks/tagged-data/0x{tag}");
    println!("Listening topic: {topic}");
    client
        .subscribe(
            vec![
                Topic::try_from(topic).unwrap()
            ],
            move |event| {
                match &event.payload {
                    MqttPayload::Json(val) => println!("{}", serde_json::to_string(&val).unwrap()),
                    MqttPayload::Block(block) => {
                        let local_block = block.clone();
                        thread::spawn(move || {
                            handle_block(local_block);
                        });
                    }
                    MqttPayload::MilestonePayload(ms) => println!("{ms:?}"),
                    MqttPayload::Receipt(receipt) => println!("{receipt:?}"),
                }
                tx.lock().unwrap().send(()).unwrap();
            },
        ).await.unwrap();
    loop {
        rx.recv().unwrap();
    }
}


fn handle_block(block: Block) {
    match block.payload() {
        None => {}
        Some(payload) => {
            match payload {
                Payload::Transaction(_) => {}
                Payload::Milestone(_) => {}
                Payload::TreasuryTransaction(_) => {}
                Payload::TaggedData(tagged_data) => {
                    let data = tagged_data.data().to_vec();
                    let mut message = decode_reader(data).unwrap();
                    let result = json::parse(message.as_str());
                    match result {
                        Ok(json) => {
                            //let message = json["message"].clone();
                            //println!("{message}");
                            //TODO Move the database path to global variable
                            let connection = Connection::open(std::env::var("DATABASE_PATH").unwrap()).unwrap();
                            connection.busy_timeout(Duration::from_secs(5)).unwrap();
                            event_handler::handle_event(json,connection);
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

