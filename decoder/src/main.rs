// DONE: read file in
// DONE: store file content as bytes
// TODO: create a data model for the instruction code
// TODO: figure out how to only parse the first byte that's for the op code
use decoder::{OpCode, Reg, D, W};
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

    let mut sorted_byte_stream: Vec<(u8, u8)> = Vec::with_capacity(file_content.len() / 2);

    let mut high_bit: u8 = 0;
    for (i, byte) in file_content.iter().enumerate() {
        if i == 0 || i % 2 == 0 {
            high_bit = *byte;
        } else {
            sorted_byte_stream.push((high_bit, *byte));
        }
    }

    println!("{:?}", sorted_byte_stream);

    for encoded_instruction in sorted_byte_stream.iter() {
        let op_codes = match OpCode::try_from(encoded_instruction.0) {
            Ok(op) => op,
            Err(err) => {
                println!("Was not able to parse the opcode");
                println!("Err: {}", err);
                panic!("aahhhhh!")
            }
        };
        let destination = match D::try_from(encoded_instruction.0) {
            Ok(d) => d,
            Err(err) => {
                println!("Was not able to parse the D field");
                println!("Err: {}", err);
                panic!("aahhhhh!")
            }
        };
        let word = match W::try_from(encoded_instruction.0) {
            Ok(w) => w,
            Err(err) => {
                println!("Was not able to parse the w field");
                println!("Err: {}", err);
                panic!("aahhhhh!")
            }
        };

        let reg = Reg::new(encoded_instruction.1);
    }
}
