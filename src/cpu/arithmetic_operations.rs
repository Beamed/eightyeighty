use super::{CPU, ExecutionError};
use super::instruction::Instruction;
use super::register::{Register, RegisterPair};

pub fn execute_instruction(cpu : &mut CPU, instruction: Instruction) -> Result<(), ExecutionError> {
    match instruction {
        Instruction::ADD(reg) => execute_add(cpu, reg, false),
        Instruction::ADC(reg) => execute_add(cpu, reg, true),
        _ => Err(ExecutionError::WrongInstructionType)
    }
}

fn execute_add(cpu: &mut CPU, reg: Register, add_carry: bool) -> Result<(), ExecutionError> {
    Ok(())
}