/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use tokio::net::TcpListener;
use tokio::prelude::*;
use super::super::log::*;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use super::parser;

static INBOUND_PORT: &str = "7777";

pub async fn start_inbound_server(queue: Arc<Mutex<VecDeque<parser::ParsePackage>>>) -> Result< (), Box<dyn std::error::Error + Send + Sync> > {
    let listener = TcpListener::bind(["0.0.0.0:", INBOUND_PORT].join("")).await?;
    print_normal("Reciever", &format!("inbound listener started successfully on port: {:?}", INBOUND_PORT));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let queue_clone = queue.clone();
        
        tokio::spawn(async move {
            let mut buffer = [0; 12000];
            loop {
                /* Keep (n) here because it returns the size of the buffer */
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        print_error("Reciever", &format!("failed to read from socket; err = {:?}", e));
                        continue;
                    }
                };

                let value = String::from_utf8(buffer[0..n].to_vec()).unwrap();
                match parser::parse_message(&value) {
                    Ok(payload) => queue_clone.lock().unwrap().push_back(payload),
                    Err(e) => {
                        print_error("Reciever", &format!("failed to parse recent messages; err = {:?}", e));
                        continue;
                    }
                };
                
            }
        });
    }

}
