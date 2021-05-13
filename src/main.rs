mod hashing;
mod inbound;
mod dispatcher;
mod log;
mod db;
mod ledger_manager;
mod profiler;
mod query_builder;

use ledger_manager::{LedgerManager};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate lazy_static;

#[allow(unused_imports)]
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

    db::print_db_entries(&mut post);
    /* Create the ledger manager. */
    let mut ledger = LedgerManager::new(Box::new(&mut post));

    /* Create a list. */
    let ledger_queue: Arc<Mutex<VecDeque<Box<String>>>> = Arc::new(Mutex::new(VecDeque::new()));

    let ledger_clone = ledger_queue.clone();
    /* When we get a job system going, we will have no need for this. */
    tokio::spawn(async move {
        let _ = match inbound::reciever::start_inbound_server().await {
            Ok(n) => n,
            Err(e) => log::print_error("Reciever", &format!("The inbound reciever failed to intilize: {:?}", e))
        };
    });
    
    loop {
        let mut ledger_locked = ledger_clone.lock().unwrap();

        drop(ledger_locked);
        std::thread::sleep(std::time::Duration::from_millis(1))
    }
}
