/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use super::inbound::parser;
use std::any::Any;

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

pub trait Parsable<'a>: Send + Sync + 'static + Any {
    fn as_any(&self) -> &dyn Any;
    //fn flatted_structure(&self) -> &str;
}

#[derive(Debug)]
pub struct Ticket<T: Send + Sync + 'static + Any + Sized> {
    pub ledger_identifier: LedgerType,
    pub ticket_identifer: String,

    pub owner: String,
    pub previous_hash: String,
    pub next_hash: String,

    pub data: Box<T>,
}

impl <T: Send + Sync + 'static + Any + Sized> Ticket<T> {
    pub fn new(led: LedgerType, tic: String, owner: String, phash: String, data: T) -> Ticket<T> {
        Ticket::<T> {ledger_identifier: led, ticket_identifer: tic, owner: owner, previous_hash: phash, next_hash: "".to_string(), data: Box::new(data)}
    }
}

#[derive(Debug)]
pub struct Location {
    pub lat: f32,
    pub long: f32,
    pub speed: f32
}

impl <'a> Parsable<'a> for Location {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl <'b, T: Send + Sync + 'static + Any + Sized> Parsable<'b> for Ticket<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/* We assume that this package has all the nessasary arguments. */
pub fn handle_ticket_creation<'a>(package: &parser::ParsePackage) -> Box<dyn Parsable<'a>>{
    match package.p_type {
        LedgerType::Location => {
            let ticket = Ticket::new(LedgerType::Location,
                package.args["identifier"].to_string(),
                package.args["owner"].to_string(),  package.args["phash"].to_string(),
                Location {lat: package.args["lat"].parse::<f32>().unwrap(), long: package.args["long"].parse::<f32>().unwrap(), speed: package.args["speed"].parse::<f32>().unwrap()});
            
                Box::new(ticket)
            
        }
        _ => {
            unreachable!()
        }
    }
}