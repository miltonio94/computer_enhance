use std::{fmt, fmt::Display, fmt::Formatter};

pub enum OpCode {
    MOV,
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

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MOV => write!(f, "MOV"),
        }
    }
}

pub enum D {
    Destination,
    Source,
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

impl D {
    fn is_register_destination(&self) -> bool {
        match *self {
            Self::Destination => true,
            Self::Source => false,
        }
    }
}

pub enum W {
    Word,
    Byte,
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
        match &format!("{byte:b}")[7..] {
            "1" => Self::Word,
            "0" => Self::Byte,
            _ => Self::Word,
        }
    }

    fn is_word_operation(&self) -> bool {
        match self {
            Self::Word => true,
            Self::Byte => false,
        }
    }
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
    NoReg,
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
        let reg = format!("{byte:b}");
        let reg = &reg[2..5];
        match reg {
            "000" => Self::AX,
            "001" => Self::CX,
            "010" => Self::DX,
            "011" => Self::BX,
            "100" => Self::SP,
            "101" => Self::BP,
            "110" => Self::SI,
            "111" => Self::DI,
            _ => Self::NoReg,
        }
    }

    fn new_for_byte_op(byte: u8) -> Self {
        let reg = format!("{byte:b}");
        let reg = &reg[2..5];
        match reg {
            "000" => Self::AL,
            "001" => Self::CL,
            "010" => Self::DL,
            "011" => Self::BL,
            "100" => Self::AH,
            "101" => Self::CH,
            "110" => Self::DH,
            "111" => Self::BH,
            _ => Self::NoReg,
        }
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::AX => write!(f, "AX"),
            Self::BX => write!(f, "BX"),
            Self::CX => write!(f, "CX"),
            Self::DX => write!(f, "DX"),
            Self::SP => write!(f, "SP"),
            Self::BP => write!(f, "BP"),
            Self::SI => write!(f, "SI"),
            Self::DI => write!(f, "DI"),
            Self::AL => write!(f, "AL"),
            Self::BL => write!(f, "BL"),
            Self::CL => write!(f, "CL"),
            Self::DL => write!(f, "DL"),
            Self::AH => write!(f, "AH"),
            Self::BH => write!(f, "BH"),
            Self::CH => write!(f, "CH"),
            Self::DH => write!(f, "DH"),
            Self::NoReg => write!(f, ""),
        }
    }
}

pub enum R_M {
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
    NoRegMem,
}

impl R_M {
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
        let reg = format!("{byte:b}");
        let reg = &reg[5..];
        match reg {
            "000" => Self::AX,
            "001" => Self::CX,
            "010" => Self::DX,
            "011" => Self::BX,
            "100" => Self::SP,
            "101" => Self::BP,
            "110" => Self::SI,
            "111" => Self::DI,
            _ => Self::NoRegMem,
        }
    }

    fn new_for_byte_op(byte: u8) -> Self {
        let reg = format!("{byte:b}");
        let reg = &reg[5..];
        match reg {
            "000" => Self::AL,
            "001" => Self::CL,
            "010" => Self::DL,
            "011" => Self::BL,
            "100" => Self::AH,
            "101" => Self::CH,
            "110" => Self::DH,
            "111" => Self::BH,
            _ => Self::NoRegMem,
        }
    }
}

impl Display for R_M {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::AX => write!(f, "AX"),
            Self::BX => write!(f, "BX"),
            Self::CX => write!(f, "CX"),
            Self::DX => write!(f, "DX"),
            Self::SP => write!(f, "SP"),
            Self::BP => write!(f, "BP"),
            Self::SI => write!(f, "SI"),
            Self::DI => write!(f, "DI"),
            Self::AL => write!(f, "AL"),
            Self::BL => write!(f, "BL"),
            Self::CL => write!(f, "CL"),
            Self::DL => write!(f, "DL"),
            Self::AH => write!(f, "AH"),
            Self::BH => write!(f, "BH"),
            Self::CH => write!(f, "CH"),
            Self::DH => write!(f, "DH"),
            Self::NoRegMem => write!(f, ""),
        }
    }
}

pub struct AssemblyCode {
    pub op_code: OpCode,
    pub destination: D,
    pub word: W,
    pub reg: Reg,
    pub rm: R_M, // this will have to become a union type
}

impl AssemblyCode {
    pub fn to_line_of_code(&self) -> String {
        if self.destination.is_register_destination() {
            return self.op_code.to_string()
                + " "
                + &self.reg.to_string()[0..]
                + " "
                + &self.rm.to_string()[0..];
        }

        return self.op_code.to_string()
            + " "
            + &self.rm.to_string()[0..]
            + " "
            + &self.reg.to_string()[0..];
    }
}
