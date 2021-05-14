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

#[derive(Debug)]
pub struct ParsePackage {
    pub args: HashMap<String, String>
}

impl ToString for ParsePackage {
    fn to_string(&self) -> String {
        let mut returnable: String = String::new();

        returnable.push_str("{\n");
        for (_, value) in self.args.iter().enumerate() {
            returnable.push_str(&format!("\t{:#?}\n", &[value.0.to_string(), value.1.to_string()].join(":")));
        }
        returnable.push_str("}\n");

        returnable
    }
}

fn gather_message_arguments(message: &str) -> HashMap<String, String> {
    let mut arugment_map: HashMap<String, String> = HashMap::new();
   
    {
        let splitted: Vec<&str> = message.split("/").collect();

        for value in splitted {
           let args: Vec<&str> = value.split(":").collect();
            arugment_map.insert(args[0].trim().to_string(), args[1].trim().to_string());
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

    if arugments.is_empty() {
        return Err(ParserError::NoArgs)
    }
    
    Ok(ParsePackage {args: arugments})
}
