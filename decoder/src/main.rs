// DONE: read file in
// DONE: store file content as bytes
// TODO: create a data model for the instruction code
use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("listing_0037_single_register_mov");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(file_err) => {
            println!("Was not able to open the file");
            println!("Err: {}", file_err);
            panic!("ahhhh!")
        }
    };

    let mut file_content = Vec::new();

    match file.read_to_end(&mut file_content) {
        Ok(r) => println!("Success: {}", r),
        Err(err) => {
            println!("Was not able to copy to buffer string");
            println!("Err: {}", err);
            panic!("aahhhhh!")
        }
    }

    for byte in file_content {
        println!("{}", format!("{byte:b}"));
    }
}
