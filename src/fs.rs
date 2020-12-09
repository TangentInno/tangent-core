/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

use super::log::*;
use std::io::Write;

static INBOUND_FILE_PATH: &str = "tmp/data/inbound/";

pub enum OutputType {
    Inbound
}

/* Goes through and makes sure each of the paths we are going to be using are instantiated. */
pub fn initilize_fs() -> Result<(), std::io::Error>{
    std::fs::create_dir("tmp/")?;
    std::fs::create_dir("tmp/data/")?;
    std::fs::create_dir("tmp/data/inbound/")?;

    Ok(())
}

pub fn write_file(filename: &str, o_type: OutputType, data: &[u8]) -> Result<(), std::io::Error>{
    let filepath: String = [INBOUND_FILE_PATH, filename, ".tan"].join("");

    let mut file: std::fs::File = match std::fs::OpenOptions::new().read(true).open(&filepath) {
        Ok(f) => f,
        Err(_) => {
            match std::fs::File::create(&filepath) {
                Ok(f) => { print_normal("FileSystem", &format!("File: {:?} created successfully.", filename)); f }
                Err(e) => return Err(e)
            }
        },
    };



    match o_type {
        OutputType::Inbound => {
            file.write(data)?;
        }
    }

    Ok(())
}