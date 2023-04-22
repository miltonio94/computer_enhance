// DONE: read file in
// DONE: store file content as bytes
// TODO: create a data model for the instruction code
// TODO: figure out how to only parse the first byte that's for the op code
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
        let _op_codes = match OpCode::try_from(byte) {
            Ok(op_code) => op_code,
            Err(err) => {
                println!("Was not able to parse the opcode");
                println!("Err: {}", err);
                panic!("aahhhhh!")
            }
        };
    }
}

enum OpCode {
    MOV,
}

enum D {
    Destination,
    Source,
}

enum W {
    Word,
    Byte,
}

enum Reg {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

struct AssemblyCode {
    opcode: OpCode,
    destination: D,
    word: W,
    reg: Reg,
    rm: Reg,
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte >> 2 {
            0b100010 => Ok(Self::MOV),
            _ => {
                let byte_ = byte >> 2;
                Err(format!("{byte_:b} is not a recognised instruction set"))
            }
        }
        // Ok(Self::MOV)
    }
}
