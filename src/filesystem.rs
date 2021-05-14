/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use std::fs::OpenOptions;
use std::io::Write;
use std::fs::create_dir_all;
use std::env::current_dir;

use super::inbound::parser::ParsePackage;

fn create_directory(identifier: &str) -> std::io::Result<()> {
    let mut user_dir = current_dir().unwrap();
    user_dir.push(identifier);

    println!("{:?}", user_dir);
    create_dir_all(user_dir)?;
    Ok(())
}

pub fn save_payload(payload: &ParsePackage) -> Result<(), String> {
    let id = payload.args.get("id").ok_or("No ID was given with the package. Defunct data recieved.")?;
    let ledger_name = payload.args.get("ledger_type").ok_or("No LedgerName was given with the package. Defunct data recieved.")?;

    let directory_name = &[id.to_string(),ledger_name.to_string()].join("/");

    create_directory(directory_name).unwrap();

    let mut file_path = current_dir().unwrap();
    file_path.push(directory_name);
    file_path.push(ledger_name);
    file_path.set_extension("tan");
    
    let mut handle = OpenOptions::new().write(true).create(true).append(true).open(file_path).unwrap();
    handle.write(payload.to_string().as_bytes()).unwrap();

    Ok(())
}

