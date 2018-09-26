pub mod register;
pub mod instruction;
pub mod condition;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
use std::ops::Add;

use self::condition::{Condition, Flag};
use self::register::{Register, RegisterPair};
use self::instruction::{Instruction};

pub type Port = u8;
pub type Address = u16;

pub struct CPU {
    memory: Vec<u8>,
    flags: HashMap<Flag, bool>,
    pc: u16,
}

const MVI_ERROR : &str = "Expected 2nd byte for MVI command";

impl CPU {
    pub fn new(mut rom_instructions: VecDeque<u8>) -> Result<CPU, Vec<Instruction>> {
        let mut instruction_vec : Vec<Instruction> = vec!();
        let mut memory_vec : Vec<u8> = vec!(65535);
        let mut addr = 0;
        while rom_instructions.len() > 0 {
            let op = rom_instructions.pop_front().expect("Error parsing opcodes. Should not have been empty.");
            //giant match below to match EVERY POSSIBLE OP
            //so, uh, don't read unless you have to
            addr += instruction.get_size();
            instruction_vec.push(instruction);
            memory_vec.push(op);
        }
        Ok(CPU {
            flags: flags,
            memory: memory_vec
        })
    }

    pub fn load_rom_into_mem(mut rom_instructions: VecDeque<u8>) {

    }
    pub fn write_output_to_file(&self, mut out: BufWriter<File>)
    {
        let mut output_buf = String::new();
        let mut cur_addr = 0;
        for instruction in &self.instructions {
            output_buf = output_buf.add(
                format!("{:#00006x}    ", cur_addr).as_str()
                )
                .add(
                    format!("{:?}", instruction).as_str()
                )
                .add("\n");
            cur_addr += instruction.get_size();
        }
        out.write_all(output_buf.as_bytes());
    }

    pub fn get_next_instruction(&mut self) {

    }
}


fn get_lxi(target_reg: RegisterPair, byte_2: Option<u8>, byte_3: Option<u8>) -> Instruction {
    let lo= byte_2.expect("Expected 2 bytes for LXI command");
    let hi = byte_3.expect("Expected 2 bytes for LXI command");
    Instruction::LXI(target_reg, (hi, lo))
}

fn create_addr(lo_byte: Option<u8>, hi_byte: Option<u8>) -> Address {
    let lo = lo_byte.expect("Expected bytes for address.");
    let hi = hi_byte.expect("Expected bytes for address");
    return ((hi as u16) << 8) + lo as u16;
}

fn get_addr(opcodes: &mut VecDeque<u8>) -> Address {
    create_addr(opcodes.pop_front(), opcodes.pop_front())
}