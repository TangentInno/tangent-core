mod hashing;
mod inbound;
mod dispatcher;
mod log;
mod profiler;
mod filesystem;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use inbound::parser::ParsePackage;

#[macro_use]
extern crate lazy_static;

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

    //let args: Vec<String> = std::env::args().collect();
    /* Display the tangent logo on run. */
    print_logo();

    /* Create a list. */
    let ledger_queue: Arc<Mutex<VecDeque<ParsePackage>>> = Arc::new(Mutex::new(VecDeque::new()));

    let ledger_clone = ledger_queue.clone();
    /* When we get a job system going, we will have no need for this. */
    tokio::spawn(async move {
        let _ = match inbound::reciever::start_inbound_server(ledger_clone).await {
            Ok(n) => n,
            Err(e) => log::print_error("Reciever", &format!("The inbound reciever failed to intilize: {:?}", e))
        };
    });
    
    let main_ledger_clone = ledger_queue.clone();
    loop {
        let mut ledger_locked = main_ledger_clone.lock().unwrap();
        
        if ledger_locked.len() != 0 {
            let payload = ledger_locked.pop_front().unwrap();
        
            match filesystem::save_payload(&payload) {
                Ok(_) => {},
                Err(msg) => {println!("{}", msg);}
            };
        }


        drop(ledger_locked);
        std::thread::sleep(std::time::Duration::from_millis(1))
    }
}
