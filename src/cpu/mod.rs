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
        let instruction = match op {
            0x00 => Instruction::NOP,
            0x01 => get_lxi(RegisterPair::BC, rom_instructions.pop_front(), rom_instructions.pop_front()),
            0x02 => Instruction::STAX(RegisterPair::BC),
            0x03 => Instruction::INX(RegisterPair::BC),
            0x04 => Instruction::INR(Register::B),
            0x05 => Instruction::DCR(Register::B),
            0x06 => Instruction::MVI(Register::B, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x07 => Instruction::RLC,
            0x09 => Instruction::DAD(RegisterPair::BC),
            0x0a => Instruction::LDAX(RegisterPair::BC),
            0x0b => Instruction::DCX(RegisterPair::BC),
            0x0c => Instruction::INR(Register::C),
            0x0d => Instruction::DCR(Register::C),
            0x0e => Instruction::MVI(Register::C, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x0f => Instruction::RRC,
            0x11 => get_lxi(RegisterPair::DE, rom_instructions.pop_front(), rom_instructions.pop_front()),
            0x12 => Instruction::STAX(RegisterPair::DE),
            0x13 => Instruction::INX(RegisterPair::DE),
            0x14 => Instruction::INR(Register::D),
            0x15 => Instruction::DCR(Register::D),
            0x16 => Instruction::MVI(Register::D, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x17 => Instruction::RAL,
            0x19 => Instruction::DAD(RegisterPair::DE),
            0x1a => Instruction::LDAX(RegisterPair::DE),
            0x1b => Instruction::DCX(RegisterPair::DE),
            0x1c => Instruction::INR(Register::E),
            0x1d => Instruction::DCR(Register::E),
            0x1e => Instruction::MVI(Register::E, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x1f => Instruction::RAR,
            0x20 => Instruction::RIM,
            0x21 => get_lxi(RegisterPair::HL, rom_instructions.pop_front(), rom_instructions.pop_front()),
            0x22 => Instruction::SHLD(get_addr(&mut rom_instructions)),
            0x23 => Instruction::INX(RegisterPair::HL),
            0x24 => Instruction::INR(Register::H),
            0x25 => Instruction::DCR(Register::H),
            0x26 => Instruction::MVI(Register::H, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x27 => Instruction::DAA,
            0x29 => Instruction::DAD(RegisterPair::HL),
            0x2a => Instruction::LHLD(get_addr(&mut rom_instructions)),
            0x2b => Instruction::DCX(RegisterPair::HL),
            0x2c => Instruction::INR(Register::L),
            0x2d => Instruction::DCR(Register::L),
            0x2e => Instruction::MVI(Register::L, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x2f => Instruction::CMA,
            0x30 => Instruction::SIM,
            0x31 => get_lxi(RegisterPair::SP, rom_instructions.pop_front(), rom_instructions.pop_front()),
            0x32 => Instruction::STA(get_addr(&mut rom_instructions)),
            0x33 => Instruction::INX(RegisterPair::SP),
            0x34 => Instruction::INR(Register::M),
            0x35 => Instruction::DCR(Register::M),
            0x36 => Instruction::MVI(Register::M, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x37 => Instruction::STC,
            0x39 => Instruction::DAD(RegisterPair::SP),
            0x3a => Instruction::LDA(get_addr(&mut rom_instructions)),
            0x3b => Instruction::DCX(RegisterPair::SP),
            0x3c => Instruction::INR(Register::A),
            0x3d => Instruction::DCR(Register::A),
            0x3e => Instruction::MVI(Register::A, rom_instructions.pop_front().expect(MVI_ERROR)),
            0x3f => Instruction::CMC,
            0x40 => Instruction::MOV(Register::B, Register::B),
            0x41 => Instruction::MOV(Register::B, Register::C),
            0x42 => Instruction::MOV(Register::B, Register::D),
            0x43 => Instruction::MOV(Register::B, Register::E),
            0x44 => Instruction::MOV(Register::B, Register::H),
            0x45 => Instruction::MOV(Register::B, Register::L),
            0x46 => Instruction::MOV(Register::B, Register::M),
            0x47 => Instruction::MOV(Register::B, Register::A),
            0x48 => Instruction::MOV(Register::C, Register::B),
            0x49 => Instruction::MOV(Register::C, Register::C),
            0x4a => Instruction::MOV(Register::C, Register::D),
            0x4b => Instruction::MOV(Register::C, Register::E),
            0x4c => Instruction::MOV(Register::C, Register::H),
            0x4d => Instruction::MOV(Register::C, Register::L),
            0x4e => Instruction::MOV(Register::C, Register::M),
            0x4f => Instruction::MOV(Register::C, Register::A),
            0x50 => Instruction::MOV(Register::D, Register::B),
            0x51 => Instruction::MOV(Register::D, Register::C),
            0x52 => Instruction::MOV(Register::D, Register::D),
            0x53 => Instruction::MOV(Register::D, Register::E),
            0x54 => Instruction::MOV(Register::D, Register::H),
            0x55 => Instruction::MOV(Register::D, Register::L),
            0x56 => Instruction::MOV(Register::D, Register::M),
            0x57 => Instruction::MOV(Register::D, Register::A),
            0x58 => Instruction::MOV(Register::E, Register::B),
            0x59 => Instruction::MOV(Register::E, Register::C),
            0x5a => Instruction::MOV(Register::E, Register::D),
            0x5b => Instruction::MOV(Register::E, Register::E),
            0x5c => Instruction::MOV(Register::E, Register::H),
            0x5d => Instruction::MOV(Register::E, Register::L),
            0x5e => Instruction::MOV(Register::E, Register::M),
            0x5f => Instruction::MOV(Register::E, Register::A),
            0x60 => Instruction::MOV(Register::H, Register::B),
            0x61 => Instruction::MOV(Register::H, Register::C),
            0x62 => Instruction::MOV(Register::H, Register::D),
            0x63 => Instruction::MOV(Register::H, Register::E),
            0x64 => Instruction::MOV(Register::H, Register::H),
            0x65 => Instruction::MOV(Register::H, Register::L),
            0x66 => Instruction::MOV(Register::H, Register::M),
            0x67 => Instruction::MOV(Register::H, Register::A),
            0x68 => Instruction::MOV(Register::L, Register::B),
            0x69 => Instruction::MOV(Register::L, Register::C),
            0x6a => Instruction::MOV(Register::L, Register::D),
            0x6b => Instruction::MOV(Register::L, Register::E),
            0x6c => Instruction::MOV(Register::L, Register::H),
            0x6d => Instruction::MOV(Register::L, Register::L),
            0x6e => Instruction::MOV(Register::L, Register::M),
            0x6f => Instruction::MOV(Register::L, Register::A),
            0x70 => Instruction::MOV(Register::M, Register::B),
            0x71 => Instruction::MOV(Register::M, Register::C),
            0x72 => Instruction::MOV(Register::M, Register::D),
            0x73 => Instruction::MOV(Register::M, Register::E),
            0x74 => Instruction::MOV(Register::M, Register::H),
            0x75 => Instruction::MOV(Register::M, Register::L),
            0x76 => Instruction::HLT,
            0x77 => Instruction::MOV(Register::M, Register::A),
            0x78 => Instruction::MOV(Register::A, Register::B),
            0x79 => Instruction::MOV(Register::A, Register::C),
            0x7a => Instruction::MOV(Register::A, Register::D),
            0x7b => Instruction::MOV(Register::A, Register::E),
            0x7c => Instruction::MOV(Register::A, Register::H),
            0x7d => Instruction::MOV(Register::A, Register::L),
            0x7e => Instruction::MOV(Register::A, Register::M),
            0x7f => Instruction::MOV(Register::A, Register::A),
            0x80 => Instruction::ADD(Register::B),
            0x81 => Instruction::ADD(Register::C),
            0x82 => Instruction::ADD(Register::D),
            0x83 => Instruction::ADD(Register::E),
            0x84 => Instruction::ADD(Register::H),
            0x85 => Instruction::ADD(Register::L),
            0x86 => Instruction::ADD(Register::M),
            0x87 => Instruction::ADD(Register::A),
            0x88 => Instruction::ADC(Register::B),
            0x89 => Instruction::ADC(Register::C),
            0x8a => Instruction::ADC(Register::D),
            0x8b => Instruction::ADC(Register::E),
            0x8c => Instruction::ADC(Register::H),
            0x8d => Instruction::ADC(Register::L),
            0x8e => Instruction::ADC(Register::M),
            0x8f => Instruction::ADC(Register::A),
            0x90 => Instruction::SUB(Register::B),
            0x91 => Instruction::SUB(Register::C),
            0x92 => Instruction::SUB(Register::D),
            0x93 => Instruction::SUB(Register::E),
            0x94 => Instruction::SUB(Register::H),
            0x95 => Instruction::SUB(Register::L),
            0x96 => Instruction::SUB(Register::M),
            0x97 => Instruction::SUB(Register::A),
            0x98 => Instruction::SBB(Register::B),
            0x99 => Instruction::SBB(Register::C),
            0x9a => Instruction::SBB(Register::D),
            0x9b => Instruction::SBB(Register::E),
            0x9c => Instruction::SBB(Register::H),
            0x9d => Instruction::SBB(Register::L),
            0x9e => Instruction::SBB(Register::M),
            0x9f => Instruction::SBB(Register::A),
            0xa0 => Instruction::ANA(Register::B),
            0xa1 => Instruction::ANA(Register::C),
            0xa2 => Instruction::ANA(Register::D),
            0xa3 => Instruction::ANA(Register::E),
            0xa4 => Instruction::ANA(Register::H),
            0xa5 => Instruction::ANA(Register::L),
            0xa6 => Instruction::ANA(Register::M),
            0xa7 => Instruction::ANA(Register::A),
            0xa8 => Instruction::XRA(Register::B),
            0xa9 => Instruction::XRA(Register::C),
            0xaa => Instruction::XRA(Register::D),
            0xab => Instruction::XRA(Register::E),
            0xac => Instruction::XRA(Register::H),
            0xad => Instruction::XRA(Register::L),
            0xae => Instruction::XRA(Register::M),
            0xaf => Instruction::XRA(Register::A),
            0xb0 => Instruction::ORA(Register::B),
            0xb1 => Instruction::ORA(Register::C),
            0xb2 => Instruction::ORA(Register::D),
            0xb3 => Instruction::ORA(Register::E),
            0xb4 => Instruction::ORA(Register::H),
            0xb5 => Instruction::ORA(Register::L),
            0xb6 => Instruction::ORA(Register::M),
            0xb7 => Instruction::ORA(Register::A),
            0xb8 => Instruction::CMP(Register::B),
            0xb9 => Instruction::CMP(Register::C),
            0xba => Instruction::CMP(Register::D),
            0xbb => Instruction::CMP(Register::E),
            0xbc => Instruction::CMP(Register::H),
            0xbd => Instruction::CMP(Register::L),
            0xbe => Instruction::CMP(Register::M),
            0xbf => Instruction::CMP(Register::A),
            0xc0 => Instruction::RETCOND(Condition::NZ),
            0xc1 => Instruction::POP(RegisterPair::BC),
            0xc2 => Instruction::JCOND(Condition::NZ, get_addr(&mut rom_instructions)),
            0xc3 => Instruction::JMP(get_addr(&mut rom_instructions)),
            0xc4 => Instruction::CCOND(Condition::NZ, get_addr(&mut rom_instructions)),
            0xc5 => Instruction::PUSH(RegisterPair::BC),
            0xc6 => Instruction::ADI(rom_instructions.pop_front().expect("ADI Opcode expected 2nd byte")),
            0xc7 => Instruction::RST(0x00),
            0xc8 => Instruction::RETCOND(Condition::Z),
            0xc9 => Instruction::RET,
            0xca => Instruction::JCOND(Condition::Z, get_addr(&mut rom_instructions)),
            0xcc => Instruction::CCOND(Condition::Z, get_addr(&mut rom_instructions)),
            0xcd => Instruction::CALL(get_addr(&mut rom_instructions)),
            0xce => Instruction::ACI(rom_instructions.pop_front().expect("ACI Opcode expected 2nd byte")),
            0xcf => Instruction::RST(0x01),
            0xd0 => Instruction::RETCOND(Condition::NC),
            0xd1 => Instruction::POP(RegisterPair::DE),
            0xd2 => Instruction::JCOND(Condition::NC, get_addr(&mut rom_instructions)),
            0xd3 => Instruction::OUT(rom_instructions.pop_front().expect("Expect byte to write to OUT")),
            0xd4 => Instruction::CCOND(Condition::NC, get_addr(&mut rom_instructions)),
            0xd5 => Instruction::PUSH(RegisterPair::DE),
            0xd6 => Instruction::SUI(rom_instructions.pop_front().expect("Expect byte to SUI")),
            0xd7 => Instruction::RST(0x02),
            0xd8 => Instruction::RETCOND(Condition::C),
            0xda => Instruction::JCOND(Condition::C, get_addr(&mut rom_instructions)),
            0xdb => Instruction::IN(rom_instructions.pop_front().expect("Expect byte (port) to read from")),
            0xdc => Instruction::CCOND(Condition::C, get_addr(&mut rom_instructions)),
            0xde => Instruction::SBI(rom_instructions.pop_front().expect("Expect byte for SBI")),
            0xdf => Instruction::RST(0x03),
            0xe0 => Instruction::RETCOND(Condition::PO),
            0xe1 => Instruction::POP(RegisterPair::HL),
            0xe2 => Instruction::JCOND(Condition::PO, get_addr(&mut rom_instructions)),
            0xe3 => Instruction::XTHL,
            0xe4 => Instruction::CCOND(Condition::PO, get_addr(&mut rom_instructions)),
            0xe5 => Instruction::PUSH(RegisterPair::HL),
            0xe6 => Instruction::ANI(rom_instructions.pop_front().expect("Expect byte for ANI")),
            0xe7 => Instruction::RST(0x04),
            0xe8 => Instruction::RETCOND(Condition::PE),
            0xe9 => Instruction::PCHL,
            0xea => Instruction::JCOND(Condition::PE, get_addr(&mut rom_instructions)),
            0xeb => Instruction::XCHG,
            0xec => Instruction::CCOND(Condition::PE, get_addr(&mut rom_instructions)),
            0xee => Instruction::XRI(rom_instructions.pop_front().expect("Expect byte for XRI")),
            0xef => Instruction::RST(0x05),
            0xf0 => Instruction::RETCOND(Condition::P),
            0xf1 => Instruction::POP_PSW,
            0xf2 => Instruction::JCOND(Condition::P, get_addr(&mut rom_instructions)),
            0xf3 => Instruction::DI,
            0xf4 => Instruction::CCOND(Condition::P, get_addr(&mut rom_instructions)),
            0xf5 => Instruction::PUSH_PSW,
            0xf6 => Instruction::ORI(rom_instructions.pop_front().expect("Expect byte for ORI")),
            0xf7 => Instruction::RST(0x06),
            0xf8 => Instruction::RETCOND(Condition::M),
            0xf9 => Instruction::SPHL,
            0xfa => Instruction::JCOND(Condition::M, get_addr(&mut rom_instructions)),
            0xfb => Instruction::EI,
            0xfc => Instruction::CCOND(Condition::M, get_addr(&mut rom_instructions)),
            0xfe => Instruction::CPI(rom_instructions.pop_front().expect("Expected argument for CPI")),
            0xff => Instruction::RST(0x07),
            _ => Instruction::NOP,//if unrecognized, default to NOOP
        };

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