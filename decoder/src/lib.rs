pub enum OpCode {
    MOV,
}

pub enum D {
    Destination,
    Source,
}

pub enum W {
    Word,
    Byte,
}

// For now we will assume that the MOD field is always 0b11

pub enum Reg {
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

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte >> 2 {
            0b_100010 => Ok(Self::MOV),
            _ => {
                let byte_ = byte >> 2;
                Err(format!("{byte_:b} is not a recognised instruction set"))
            }
        }
    }
}

impl TryFrom<u8> for D {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        // For now I will keep a try_from impl,
        // but I don't think it makes too much sense
        match (byte >> 1) << 6 {
            0b_1 => Ok(Self::Destination),
            0b_0 => Ok(Self::Source),
            _ => Err(format!("Could not get the D field from {byte:b}")),
        }
    }
}

impl TryFrom<u8> for W {
    type Error = String;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match (byte << 7) >> 7 {
            0b_1 => Ok(Self::Word),
            0b_0 => Ok(Self::Byte),
            _ => Err(format!("Could not get the D field from {byte:b}")),
        }
    }
}

impl W {
    pub fn new(byte: u8) -> Self {
        if byte << 7 == 0b_1 {
            return Self::Word;
        }
        return Self::Byte;
    }

    fn is_word_operation(&self) -> bool {
        match self {
            Self::Word => true,
            Self::Byte => false,
        }
    }
}

impl Reg {
    // | Register/Memory field encoding |       |       |
    // |--------------------------------+-------+-------|
    // |                  when MOD = 11 |       |       |
    // |--------------------------------+-------+-------|
    // |                            R/M | W = 0 | W = 1 |
    // |--------------------------------+-------+-------|
    // |                            000 | AL    | AX    |
    // |                            001 | CL    | CX    |
    // |                            010 | DL    | DX    |
    // |                            011 | BL    | BX    |
    // |                            100 | AH    | SP    |
    // |                            101 | CH    | BP    |
    // |                            110 | DH    | SI    |
    // |                            111 | BH    | DI    |
    // |--------------------------------+-------+-------|

    pub fn new(byte: u8) -> Self {
        let w = W::new(byte);
        if w.is_word_operation() {
            Self::new_for_word_op(byte)
        } else {
            Self::new_for_byte_op(byte)
        }
    }

    fn new_for_word_op(byte: u8) -> Self {
        let reg = (byte << 2) >> 3;
        println!(
            "Byte: {}\t  Reg: {}",
            format!("{byte:b}"),
            format!("{reg:b}")
        );
        Self::AH
    }

    fn new_for_byte_op(byte: u8) -> Self {
        let reg = (byte >> 3) << 2;
        println!(
            "Byte: {}\t  Reg: {}",
            format!("{byte:b}"),
            format!("{reg:b}")
        );
        Self::AL
    }
}

struct AssemblyCode {
    op_code: OpCode,
    destination: D,
    word: W,
    reg: Reg,
    rm: Reg, // this will have to become a union type
}
