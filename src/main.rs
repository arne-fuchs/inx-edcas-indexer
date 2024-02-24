use std::{io, process};
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use ::tonic::codegen::tokio_stream::StreamExt;
use ::tonic::transport::Uri;
use base64::Engine;
use base64::engine::general_purpose;
use flate2::read::ZlibDecoder;
use iota_sdk::client::mqtt::Topic;
use iota_sdk::packable::PackableExt;
use iota_sdk::types::block::Block;
use iota_sdk::types::block::payload::Payload;
use iota_sdk::types::block::signature::Ed25519Signature;
use rustc_hex::FromHex;
use tokio::sync::Mutex;
use tokio_postgres::NoTls;

use tonic::transport::Channel;

pub use self::proto::inx_client as client;

mod event_handler;

pub mod proto {
    #![allow(missing_docs)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("inx");
}

/// Re-exports of [`tonic`] types.
pub mod tonic {
    pub use tonic::*;
}

#[tokio::main]
async fn main() {
    println!("Getting ready...");

    let inx_address = std::env::var("INX_ADDRESS").unwrap();
    println!("Inx_Address: {}", &inx_address);
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

    let (postgres_client, connection) = tokio_postgres::connect(connection_string.as_str(), NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("connection error: {}", e);
        }
    });
    println!("Connected");
    println!("Running create Tables");
    let script = std::fs::read_to_string("createTables.sql").unwrap();
    postgres_client.batch_execute(&script).await.unwrap();
    let shareable_client = Arc::new(Mutex::new(postgres_client));
    println!("Done!");

    println!("Connecting to inx...");

    let inx_url: Uri = {
        let mut string = String::from("http://");
        string.push_str(inx_address.as_str());
        string.clone().as_str()
    }.parse().unwrap();

    let inx_channel = {
        let mut result = Channel::builder(inx_url.clone()).connect().await;
        while result.is_err(){
            println!("Trying to connect to inx... ({})({})", &inx_url,&result.err().unwrap());
            tokio::time::sleep(Duration::from_secs(5)).await;
            result = Channel::builder(inx_url.clone()).connect().await;
        }

        result.unwrap()
    };
    let mut inx_client = client::InxClient::new(inx_channel);

    let mut response_node_status = inx_client.read_node_status(
        proto::NoParams{}
    ).await.expect("Request failed");

    let mut node_status = response_node_status.into_inner();
    while !node_status.is_healthy && !node_status.is_synced {
        println!("Waiting for node to be healthy and synced...");
        println!("Health: {}\t Synced: {}", &node_status.is_healthy,&node_status.is_synced);
        tokio::time::sleep(Duration::from_secs(5)).await;
        response_node_status = inx_client.read_node_status(
            proto::NoParams{}
        ).await.expect("Failed requesting node status");

        node_status = response_node_status.into_inner();
    }

    //Node is synced and healthy at this point
    println!("Connected and healthy!");

    let response_listen_blocks = inx_client.listen_to_blocks(
        proto::NoParams{}
    ).await.expect("Failed listening to blocks");
    let mut block_stream = response_listen_blocks.into_inner();
    loop {
        let client_clone = shareable_client.clone();
        let stream_block = block_stream.next().await;
        match stream_block {
            None => {
                println!("Couldn't find block");
            }
            Some(block_result) => {
                match block_result {
                    Ok(proto_block) => {
                        //println!("0x{}",hex::encode(block.block_id.unwrap().id));
                        match proto_block.block {
                            None => {
                                eprintln!("No raw block found");
                            }
                            Some(raw_block) => {
                                let block_unpack_result = Block::unpack_unverified(raw_block.data);
                                match block_unpack_result {
                                    Ok(block) => {
                                            match block.payload() {
                                                None => {
                                                    eprintln!("Couldn't found payload for block")
                                                }
                                                Some(payload) => {
                                                    let payload_clone = payload.clone();
                                                    tokio::spawn( async move {
                                                        handle_block(payload_clone,client_clone.clone()).await;
                                                    });
                                                   
                                                }
                                            }
                                    }
                                    Err(err) => {
                                        eprintln!("Unpacking raw block failed: {}", err);
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error getting block: {}", err);
                    }
                }
            }
        }
    }
}

async fn handle_block(payload: Payload,client: Arc<Mutex<tokio_postgres::Client>>) {
    match payload {
        Payload::Transaction(_) => {}
        Payload::Milestone(_) => {}
        Payload::TreasuryTransaction(_) => {}
        Payload::TaggedData(tagged_data) => {
            let result = json::parse(String::from_utf8(tagged_data.data().to_vec()).unwrap().as_str());
            match result {
                Ok(json) => {
                    let tag = String::from_utf8(tagged_data.tag().to_vec()).unwrap();
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

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

