use std::convert::From;

pub type RegisterOp = u8;
pub type RegisterPairOp = u8;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
}

impl From<RegisterOp> for Register {
    fn from(op: RegisterOp) -> Self {
        match op {
            0b000 => Register::B,
            0b001 => Register::C,
            0b010 => Register::D,
            0b011 => Register::E,
            0b100 => Register::H,
            0b101 => Register::L,
            0b110 => Register::M,
            0b111 => Register::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

impl From<RegisterPairOp> for RegisterPair {
    fn from(op: RegisterPairOp) -> Self {
        match op {
            0b00 => RegisterPair::BC,
            0b01 => RegisterPair::DE,
            0b10 => RegisterPair::HL,
            0b11 => RegisterPair::SP,
            _ => panic!("Invalid register pair requested: {}", op)
        }
    }
}

impl From<Register> for RegisterOp {
    fn from(reg: Register) -> Self {
        match reg {
            Register::A => 0b111,
            Register::B => 0b000,
            Register::C => 0b001,
            Register::D => 0b010,
            Register::E => 0b011,
            Register::H => 0b100,
            Register::L => 0b101,
            Register::M => 0b110,
        }
    }
}

impl From<RegisterPair> for RegisterPairOp {
    fn from(reg: RegisterPair) -> Self {
        match reg {
            RegisterPair::BC => 0b00,
            RegisterPair::DE => 0b01,
            RegisterPair::HL => 0b10,
            RegisterPair::SP => 0b11
        }
    }
}