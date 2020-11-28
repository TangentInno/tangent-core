/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use tokio::net::TcpListener;
use tokio::prelude::*;
use super::super::log::*;

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
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        print_error("Reciever", &format!("failed to read from socket; err = {:?}", e));
                        return;
                    }
                };

                print_normal("Reciever", &format!("[Reviever] Retrived: {:?}", n));
            }
        });
    }
}