use std::convert::From;

pub type ConditionOpCode = i8;
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Condition {
    NZ,
    Z,
    NC,
    C,
    PO,
    PE,
    P,
    M,
}

impl From<ConditionOpCode> for Condition  {
    fn from(op: ConditionOpCode) -> Self {
        match op {
            0b000 => Condition::NZ,
            0b001 => Condition::Z,
            0b010 => Condition::NC,
            0b011 => Condition::C,
            0b100 => Condition::PO,
            0b101 => Condition::PE,
            0b110 => Condition::P,
            0b111 => Condition::M,
            _ => unreachable!(),
        }
    }
}

impl From<Condition> for ConditionOpCode {
    fn from(op: Condition) -> Self {
        match op {
            Condition::NZ   => 0b000,
            Condition::Z    => 0b001,
            Condition::NC   => 0b010,
            Condition::C    => 0b011,
            Condition::PO   => 0b100,
            Condition::PE   => 0b101,
            Condition::P    => 0b110,
            Condition::M    => 0b111,
            _ => unreachable!(),
        }
    }
}