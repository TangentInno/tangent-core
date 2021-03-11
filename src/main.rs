mod hashing;
mod inbound;
mod dispatcher;
mod ticket;
mod log;
mod db;
mod ledger_manager;
mod profiler;

use ledger_manager::{LedgerManager};
use std::collections::VecDeque;
use ticket::{ Parsable};
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate postgres;

fn print_logo() {
    println!(r"
    _________  ________  ________   ________  _______   ________   _________   
   |\___   ___\\   __  \|\   ___  \|\   ____\|\  ___ \ |\   ___  \|\___   ___\ 
   \|___ \  \_\ \  \|\  \ \  \\ \  \ \  \___|\ \   __/|\ \  \\ \  \|___ \  \_| 
        \ \  \ \ \   __  \ \  \\ \  \ \  \  __\ \  \_|/_\ \  \\ \  \   \ \  \  
         \ \  \ \ \  \ \  \ \  \\ \  \ \  \|\  \ \  \_|\ \ \  \\ \  \   \ \  \ 
          \ \__\ \ \__\ \__\ \__\\ \__\ \_______\ \_______\ \__\\ \__\   \ \__\
           \|__|  \|__|\|__|\|__| \|__|\|_______|\|_______|\|__| \|__|    \|__| 
    ");
}

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();
    /* Display the tangent logo on run. */
    print_logo();
    
    /* Instantiate the database. */
    let mut post = db::established(&args[1]);

    /* Create the ledger manager. */
    let mut ledger = LedgerManager::new(Box::new(&mut post));

    /* Create a list. */
    let ledger_queue: Arc<Mutex<VecDeque<Box<dyn Parsable>>>> = Arc::new(Mutex::new(VecDeque::new()));

    let ledger_clone = ledger_queue.clone();
    /* When we get a job system going, we will have no need for this. */
    tokio::spawn(async move {
        let _ = match inbound::reciever::start_inbound_server(ledger_queue).await {
            Ok(n) => n,
            Err(e) => log::print_error("Reciever", &format!("The inbound reciever failed to intilize: {:?}", e))
        };
    });
    loop {
        let mut ledger_locked = ledger_clone.lock().unwrap();

        if !ledger_locked.is_empty() {
            ledger.manage_incoming_ticket::<&dyn Parsable>(ledger_locked.pop_front().unwrap());
        }

        drop(ledger_locked);
        std::thread::sleep(std::time::Duration::from_millis(1))
    }
}
