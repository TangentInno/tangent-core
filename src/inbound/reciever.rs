/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use tokio::net::TcpListener;
use tokio::prelude::*;
use super::super::log::*;

use super::super::ticket::{handle_ticket_creation, Parsable};
use super::parser;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

static INBOUND_PORT: &str = "7777";

pub async fn start_inbound_server(ledger_queue: Arc<Mutex<VecDeque<Box<dyn Parsable<'static>>>>>)  -> Result< (), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let listener = TcpListener::bind(["0.0.0.0:", INBOUND_PORT].join("")).await?;
    print_normal("Reciever", &format!("inbound listener started successfully on port: {:?}", INBOUND_PORT));

    loop {
        let (mut socket, _) = listener.accept().await?;

        let ledger_queue_clone = ledger_queue.clone();
        tokio::spawn(async move {

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

                let value = String::from_utf8(buffer[0..n].to_vec()).unwrap();
                let message = match parser::parse_message(&value) {
                    Ok(tic) => tic,
                    Err(e) => {
                        print_error("Reciever", &format!("failed to parse recent messages; err = {:?}", e));
                        return;
                    }
                };
                
                let ticket: Box<dyn Parsable> = handle_ticket_creation(&message);
                ledger_queue_clone.lock().unwrap().push_back(ticket);
            }
        });
    }

}