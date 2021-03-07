/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use super::inbound::parser;
use super::hashing::sha256;

lazy_static! {
   pub static ref TicketTags: std::collections::HashMap<&'static str, &'static [&'static str; 3]> = {
        let mut m = std::collections::HashMap::new();
        m.insert("Location", &["lat", "long", "speed"]);
        m.insert("Pizza", &["cheese", "sauce", "tears"]);
        m
    };
}

#[derive(Debug)]
pub enum LedgerType {
    Location,
    Pizza
}

impl LedgerType {
    pub fn from_str(value: &str) -> LedgerType {
        match value {
            "Location" => LedgerType::Location,
            "Pizza" => LedgerType::Pizza,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Ticket<T> {
    pub ledger_identifier: LedgerType,
    pub ticket_identifer: String,

    pub owner: String,
    pub previous_hash: String,
    pub next_hash: String,

    pub data: T,
}

impl <T> Ticket<T> {
    pub fn new(led: LedgerType, tic: String, owner: String, phash: String, nhash: String, data: T) -> Ticket<T>{
        Ticket::<T> {ledger_identifier: led, ticket_identifer: tic, owner: owner, previous_hash: phash, next_hash: nhash, data: data}
    }
}

#[derive(Debug)]
pub struct Location {
    pub lat: f32,
    pub long: f32,
    pub speed: f32
}

pub fn handle_ticket_creation(package: &parser::ParsePackage) {
    match package.pType {
        LedgerType::Location => {
            Ticket::new(LedgerType::Location,
                sha256::generate_sha256_hash("Yup").to_string(),
                package.args["owner"].to_string(),  "".to_string(),  "".to_string(),
                Location {lat: package.args["lat"].parse::<f32>().unwrap(), long: package.args["long"].parse::<f32>().unwrap(), speed: package.args["speed"].parse::<f32>().unwrap()});
        }
        _ => {

        }
    }
}