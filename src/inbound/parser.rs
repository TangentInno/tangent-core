/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParserError {
    RequestEmpty,
    InvalidInput,
    NoArgs
}

pub struct ParsePackage<'a> {
    pub args: HashMap<&'a str, &'a str>,
}

fn gather_message_arguments(message: &str) -> HashMap<&str, &str> {
    let mut arugment_map: HashMap<&str, &str> = HashMap::new();
   
    {
        let splitted: Vec<&str> = message.split("/").collect();

        for value in splitted {
           let args: Vec<&str> = value.split(":").collect();
            arugment_map.insert(args[0].trim(), args[1].trim());
        }
    }

    arugment_map
}

/* TODO: Really extensify this to make sure it matches the format we use for parsing. */
fn validate_input(message: &str) -> bool {
    message.contains(":")
}

pub fn parse_message (message: &str) -> Result<ParsePackage, ParserError> {

    if message.is_empty() {
        return Err(ParserError::RequestEmpty)
    }

    if !validate_input(message) {
        return Err(ParserError::InvalidInput)
    }

    let arugments = gather_message_arguments(message);

    let keys: Vec<&str> = arugments.keys().cloned().collect();

    if arugments.is_empty() {
        return Err(ParserError::NoArgs)
    }
    
    Ok(ParsePackage {args: arugments})
}
