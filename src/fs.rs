/*
*
*	Author: Austin Mullins
*	Copyright: Tangent
*
*/

static INBOUND_FILE_PATH: &str = "tmp/data/inbound/";

pub enum OutputType {
    Inbound
}

/* Goes through and makes sure each of the paths we are going to be using are instantiated. */
pub fn initilize_fs() -> Result<(), std::io::Error>{
    std::fs::create_dir("tmp/")?;
    std::fs::create_dir("tmp/data/");
    std::fs::create_dir("tmp/data/inbound/");

    Ok(())
}

pub fn write_file(filename: &str, o_type: OutputType, data: &[u8]) -> Result<(), std::io::Error>{
    match o_type {
        OutputType::Inbound => {
            std::fs::write([INBOUND_FILE_PATH, filename, ".tan"].join(""), data)?;
        }
        _ => {}
    }

    Ok(())
}