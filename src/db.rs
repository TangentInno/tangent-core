use postgres::{Client, NoTls, Row};
use super::log;

use chrono;

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

struct TableData<'a> {
    pub args: Vec<&'a str>,
}
