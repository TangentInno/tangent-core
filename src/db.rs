use postgres::{Client, NoTls, Row};
use super::ticket::{Ticket, Location};
use super::log;

use chrono;

/*
pub struct LocationTicket {
    pub identifier: String,
    pub owner: String,
    pub previous_hash: Option<String>,
    pub next_hash: Option<String>,
    pub lat: f32,
    pub long: f32,
    pub speed: f32,
    pub creation_date: String,
}
*/

pub fn established(postgres_pass: &str) -> Client {
    Client::connect(&("host=tangentinno.com user=postgres dbname=tangent_core password=".to_string() + postgres_pass), NoTls).expect(&format!("Error connecting to {}", "10.0.0.73:5432"))
}

pub fn get_chain_entries(db: &mut Client, next_id: &str, entry_vec: &mut Vec<(f32, f32)> ){
    let entries: Vec<Row> = db.query("SELECT * FROM public.location_data WHERE identifier=$1", &[&next_id]).unwrap();

    if entries.len() == 0 { return; }

    let next_hash: String = entries[0].get("next_hash");
    if !next_hash.is_empty() {
        get_chain_entries(db, &next_hash, entry_vec);
    }

    let lat: f32 = entries[0].get("lat");
    let long: f32 = entries[0].get("long");

    entry_vec.push((lat, long));
}
    

pub fn print_db_entries(db: &mut Client) {
    let entries: Vec<Row> = db.query("SELECT * FROM public.location_data WHERE previous_hash=''", &[]).unwrap();

    log::print_normal("Database", &format!("Entries found: {:?}", entries.len()));

    for entry in entries {
        let id: String = entry.get("identifier");
        log::print_normal("Database", &format!("Entries identity: {:?}", id));

        let mut entry_data: Vec<(f32, f32)> = Vec::new();
        get_chain_entries(db, &id, &mut entry_data);
        
        for (i, entry) in entry_data.iter().enumerate() {
            println!("{:#?}, {:#?} {{pos {:#?}}} ", entry.0, entry.1, i);
        }
    }
}

pub fn check_db_ticket_existance(db: &mut Client, id: &str) -> bool {
   match db.query("SELECT * FROM public.location_data WHERE identifier=$1;", &[&id])
   {
       Ok(rows) => {return rows.len() > 0},
       Err(e) => {log::print_error("Postgres", &format!("There was an error checking the database: {:?}", e)); return false}
   }
}

fn update_previous_hash<T: Send + Sync + 'static + Sized>(db: &mut Client, ticket: &Ticket<T>) {
    if check_db_ticket_existance(db, &ticket.previous_hash) {

        // Update the past ticket's next_has with our new one.
        // TODO: Make this safe.
        match db.execute("UPDATE public.location_data SET next_hash = $1 WHERE identifier = $2;", &[&ticket.ticket_identifer, &ticket.previous_hash]) {
            Ok(_) => {},
            Err(e) => {log::print_error("Postgres", &format!("There was an updating a ticket in the database: {:?}", e));}
        };

    }
}

pub fn add_ticket(db: &mut Client, ticket: &Ticket<Location>) -> bool {

    update_previous_hash(db, ticket);

    let now = chrono::Utc::now();
    let time: String = format!("{}", now);

    match db.execute("INSERT INTO public.location_data (identifier, owner, previous_hash, next_hash, lat, long, speed, creation_date) 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8);", &[&ticket.ticket_identifer, &ticket.owner, &ticket.previous_hash, &ticket.next_hash,
        &ticket.data.lat, &ticket.data.long, &ticket.data.speed,  &time]) {
            Ok(_) => true,
            Err(e) => {log::print_error("Postgres", &format!("There was an error putting a Location Ticket into the database: {:?}", e)); false}
        }

}

struct TableData<'a> {
    pub args: Vec<&'a str>,
}
