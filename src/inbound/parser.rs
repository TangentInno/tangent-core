/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use super::super::ticket::{Ticket, LedgerType, Location, TicketTags};
use super::super::hashing;


use std::collections::HashMap;

#[derive(Debug)]
pub enum ParserError {
    RequestEmpty,
    NoArgs
}

pub struct ParsePackage<'a> {
    pub args: HashMap<&'a str, &'a str>,
    pub pType: LedgerType,
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

fn infer_ledger_type_from_args(args: &Vec<&str>) -> String {
    let mut counter: [i8; 10] = [0; 10];
    for (index, (_, tags)) in TicketTags.iter().enumerate() {
        for (_, tag) in tags.iter().enumerate() {
            for (_, arg) in args.iter().enumerate() {
                if arg == tag {
                    counter[index] += 1;
                }
            }
        }
    }

    let mut winning_index = 0;
    for (i, v) in counter.iter().enumerate() {
        if i == 0 { continue; }
        if v > &counter[i - 1] {
            winning_index = i;
        }
    }

    let mut winning_ledger: &str = "";
    for (index, (ledger, _)) in TicketTags.iter().enumerate() {
        if index == winning_index {
            winning_ledger = ledger;
        }
    }

    String::from(winning_ledger)
}

pub fn parse_message (message: &str) -> Result<ParsePackage, ParserError> {

    if message.is_empty() {
        return Err(ParserError::RequestEmpty)
    }

    let arugments = gather_message_arguments(message);

    let keys: Vec<&str> = arugments.keys().cloned().collect();

    let infered_type = infer_ledger_type_from_args(&keys);
    
    if arugments.is_empty() {
        return Err(ParserError::NoArgs)
    }
    
    Ok(ParsePackage {args: arugments, pType: LedgerType::from_str(&infered_type) })
}