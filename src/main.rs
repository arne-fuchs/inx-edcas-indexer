use std::{io, process};
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc::channel;

use base64::Engine;
use base64::engine::general_purpose;
use flate2::read::ZlibDecoder;
use iota_sdk::client::Client;
use iota_sdk::client::mqtt::{MqttEvent, MqttPayload, Topic};
use iota_sdk::types::block::BlockDto;
use iota_sdk::types::block::payload::dto::PayloadDto;
use iota_sdk::types::block::signature::Ed25519Signature;
use rustc_hex::FromHex;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;

mod event_handler;

#[tokio::main]
async fn main() {
    println!("Getting ready...");

    let node_url = std::env::var("NODE_URL").unwrap();
    println!("Node_url: {}", &node_url);
    let username = std::env::var("POSTGRES_USER").unwrap();
    let password = std::env::var("POSTGRES_PASSWORD").unwrap();
    let db_host = std::env::var("DATABASE_HOST").unwrap();
    println!("DB_Host: {}", &db_host);
    let db_port = std::env::var("DATABASE_PORT").unwrap();
    println!("DB_Port: {}", &db_port);
    let database = std::env::var("DATABASE_NAME").unwrap_or("edcas".to_string());
    println!("Database: {}", &database);
    let pow_worker_count = usize::from_str(std::env::var("NUM_OF_WORKERS").unwrap_or("4".to_string()).as_str()).unwrap();

    let tag_env = std::env::var("TAGS").unwrap().to_string();
    let tags: Vec<&str> = tag_env.split(",").collect();
    println!("Tags: {:?}", &tags);


    let topics = tags.iter().map(|tag| Topic::new(format!("blocks/tagged-data/0x{}",hex::encode(tag))).unwrap());
    println!("Listening topics: {:?}",topics);

    println!("Connect to database");
    let connection_string = format!("postgresql://{username}:{password}@{db_host}:{db_port}/{database}");

    let (client, connection) = tokio_postgres::connect(connection_string.as_str(), NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("Connected");
    println!("Running create Tables");
    let script = std::fs::read_to_string("createTables.sql").unwrap();
    client.batch_execute(&script).await.unwrap();
    println!("Done");

    println!("Connecting to node");
    let shareable_client = Arc::new(Mutex::new(client));

    let node = Client::builder()
        .with_node(node_url.as_str()).unwrap()
        .with_pow_worker_count(pow_worker_count)
        .with_local_pow(true)
        .finish().await.unwrap();

    let (tx, rx) = channel();
    let tx = Arc::new(std::sync::Mutex::new(tx));

    let mut event_rx = node.mqtt_event_receiver().await;
    tokio::spawn(async move {
        while event_rx.changed().await.is_ok() {
            let event = event_rx.borrow();
            if *event == MqttEvent::Disconnected {
                println!("mqtt disconnected");
                process::exit(1);
            }
        }
    });

    println!("Starting MQTT");
    node
        .subscribe(
            topics,
            move |event| {
                match &event.payload {
                    MqttPayload::Json(val) => println!("{}", serde_json::to_string(&val).unwrap()),
                    MqttPayload::Block(block) => {
                        let local_block = block.clone();
                        let client_clone = shareable_client.clone();
                        tokio::spawn( async move {
                            handle_block(local_block,client_clone).await;
                        });
                    }
                    MqttPayload::MilestonePayload(ms) => println!("{ms:?}"),
                    MqttPayload::Receipt(receipt) => println!("{receipt:?}"),
                    _ => {}
                }
                tx.lock().unwrap().send(()).unwrap();
            },
        ).await.unwrap();
    println!("Done");
    loop {
        rx.recv().unwrap();
    }
}


async fn handle_block(block: BlockDto,client: Arc<Mutex<tokio_postgres::Client>>) {
    match block.payload {
        None => {}
        Some(payload) => {
            match payload {
                PayloadDto::Transaction(_) => {}
                PayloadDto::Milestone(_) => {}
                PayloadDto::TreasuryTransaction(_) => {}
                PayloadDto::TaggedData(tagged_data) => {
                    let result = json::parse(String::from_utf8(tagged_data.data.to_vec()).unwrap().as_str());
                    match result {
                        Ok(json) => {
                            let tag = String::from_utf8(tagged_data.tag.to_vec()).unwrap();
                            if !std::env::var("TAGS").unwrap().contains(&tag) {
                                return;
                            }
                            //println!("{}",&json);

                            let data = general_purpose::STANDARD.decode(json["message"].as_str().unwrap()).unwrap();

                            let p_key = json["public_key"].to_string();
                            let pub_key_bytes: Vec<u8> = json["public_key"].as_str().unwrap()[2..].from_hex().unwrap();
                            let mut pub_key: [u8;32] = [0u8;32];
                            pub_key[0..32].copy_from_slice(&pub_key_bytes[0..32]);

                            let sig_bytes: Vec<u8> = json["signature"].as_str().unwrap()[2..].from_hex().unwrap();
                            let mut sig: [u8;64] = [0u8;64];
                            sig[0..64].copy_from_slice(&sig_bytes[0..64]);

                            let sig = Ed25519Signature::try_from_bytes(pub_key,sig).unwrap();

                            if sig.verify(data.as_slice()) {
                                if client.lock().await.is_closed(){
                                    process::exit(20);
                                }
                                //let message = json["message"].clone();
                                //println!("{message}");
                                //println!("{}",&json);
                                //language=postgresql
                                let sql = "INSERT INTO pid VALUES ($1) ON CONFLICT (pkey) DO NOTHING;";
                                client.lock().await.execute(sql,&[&p_key]).await.unwrap();
                                match json::parse(decode_reader(data).unwrap().as_str()) {
                                    Ok(json) => {
                                        event_handler::handle_event(json,client).await;
                                    }
                                    Err(_) => {
                                        println!("Unable to parse json!");
                                    }
                                }
                            } else {
                                println!("Signature verification failed.");
                            }
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

