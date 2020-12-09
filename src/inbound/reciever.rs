/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use tokio::net::TcpListener;
use tokio::prelude::*;
use super::super::log::*;
use super::super::fs;
use super::super::hashing;

static INBOUND_PORT: &str = "7777";

pub async fn start_inbound_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(["0.0.0.0:", INBOUND_PORT].join("")).await?;
    print_normal("Reciever", &format!("inbound listener started successfully on port: {:?}", INBOUND_PORT));

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn( async move {
            let mut buffer: [u8; 1024] = [0; 1024];

            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        print_error("Reciever", &format!("failed to read from socket; err = {:?}", e));
                        return;
                    }
                };

                let filename = "output";//&hashing::sha256::generate_sha256_hash(&format!("{:?}", std::time::Instant::now()));
                match fs::write_file(filename, fs::OutputType::Inbound, &buffer[0..n]) {
                    Ok(_) => print_normal("FileSystem", &format!("File: {:?} created successfully.", filename)),
                    Err(e) => print_error("FileSystem", &format!("There was an issue making the inbound file: {:?} \n {:?}", filename, e))
                }
                print_normal("Reciever", &format!("[Reviever] Retrived: {:?}", String::from_utf8(buffer[0..n].to_vec()).unwrap()));
            }
        });
    }
}