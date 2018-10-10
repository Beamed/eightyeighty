use std::convert::From;

pub type ConditionOpCode = i8;
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ConditionOp {
    NZ,
    Z,
    NC,
    C,
    PO,
    PE,
    P,
    M,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Condition {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
}

impl Condition {
    pub fn new() -> Condition {
        Condition {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
        }
    }
}

impl From<ConditionOpCode> for ConditionOp {
    fn from(op: ConditionOpCode) -> Self {
        match op {
            0b000 => ConditionOp::NZ,
            0b001 => ConditionOp::Z,
            0b010 => ConditionOp::NC,
            0b011 => ConditionOp::C,
            0b100 => ConditionOp::PO,
            0b101 => ConditionOp::PE,
            0b110 => ConditionOp::P,
            0b111 => ConditionOp::M,
            _ => unreachable!(),
        }
    }
}

impl From<ConditionOp> for ConditionOpCode {
    fn from(op: ConditionOp) -> Self {
        match op {
            ConditionOp::NZ   => 0b000,
            ConditionOp::Z    => 0b001,
            ConditionOp::NC   => 0b010,
            ConditionOp::C    => 0b011,
            ConditionOp::PO   => 0b100,
            ConditionOp::PE   => 0b101,
            ConditionOp::P    => 0b110,
            ConditionOp::M    => 0b111,
            _ => unreachable!(),
        }
    }
}