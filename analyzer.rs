use std::io::*;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let file_path = std::env::args().nth(1).expect("There was no filepath given");

    for entry in std::fs::read_dir(file_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let mut file = File::open(path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

	let found_string_index = match buffer.find("d") {
	    Some(s) => s,
	    None => { continue; }
	};

	if found_string_index != 0 { 
	    println!("{:?}", &buffer.split_at(found_string_index).1[2..22]);    
	}    
    }
}
