pub enum OpCode {
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
