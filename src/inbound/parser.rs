use super::super::ticket_types::{Location};
use super::super::ledger::{Ticket, LedgerType};
use super::super::hashing;


use std::collections::HashMap;

#[derive(Debug)]
pub enum ParserError {
    RequestEmpty,
    NoArgs
}

fn gather_message_arguments(message: &str) -> HashMap<&str, &str> {
    let mut arugment_map: HashMap<&str, &str> = HashMap::new();
   
    {
        let splitted: Vec<&str> = message.split("/").collect();

        for value in splitted {
           let args: Vec<&str> = value.split(":").collect();
            arugment_map.insert(args[0], args[1]);
        }
    }

    arugment_map
}

pub fn parse_message(message: &str) -> Result<Ticket<Location>, ParserError> {

    if message.is_empty() {
        return Err(ParserError::RequestEmpty)
    }

    let arugments = gather_message_arguments(message);

    if arugments.is_empty() {
        return Err(ParserError::NoArgs)
    }
    
    return Ok(Ticket {

        ledger_identifier: LedgerType::Location,
        ticket_identifer: hashing::sha256::generate_sha256_hash("Yup").to_string(),

        // Get owner from message.
        owner: arugments["owner"].to_string(),
        previous_hash: "".to_string(),
        next_hash: "".to_string(),
    
        data: Location {lat: arugments["lat"].parse::<f32>().unwrap(), long: arugments["long"].parse::<f32>().unwrap(), speed: arugments["speed"].parse::<f32>().unwrap()},
        
    })
}