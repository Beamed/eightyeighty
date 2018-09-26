mod cpu;

use std::io;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::ops::Add;
use std::collections::HashMap;

use cpu::CPU;
use cpu::instruction::Instruction;
use cpu::register::{Register, RegisterPair, RegisterOp, RegisterPairOp};

fn main() {
    println!("Time for some nostalgia!");
    let mut path_name = String::new();
    println!("Please put in the file we're disassembling today.");
    if let Err(error) = io::stdin().read_line(&mut path_name) {
        eprintln!("Please input a valid string.");
        return;
    }
    path_name = path_name.trim().to_string();
    let out_path_name = path_name.clone().add(".out");
    let path = Path::new(&path_name);
    let file = File::open(path).expect(format!("Unable to open invalid file path: {}", path.to_str().unwrap()).as_str());
    let mut reader = BufReader::new(file);

    let cpu = load_cpu_with_instructions_from_file(reader);
    let output_file_path = Path::new(&out_path_name);
    let mut out_file = File::create(output_file_path).expect("Unable to write output file, aborting.");

    cpu.write_output_to_file(BufWriter::new(out_file));
}

pub fn load_cpu_with_instructions_from_file(mut reader: BufReader<File>) -> CPU
{
    let mut opcode_buffer : Vec<u8> = vec!();
    println!("Reading file into system!");
    reader.read_to_end(&mut opcode_buffer).expect("Unable to read from file. Aborting.");
    let mut opcodes = VecDeque::from_iter(opcode_buffer.into_iter());
    println!("Successfully read {} instructions, decoding..", opcodes.len());
    match CPU::new(opcodes) {
        Ok(cpu) => cpu,
        Err(decoded_instructions) => {
            println!("Unable to disassemble. Here is the code before the failed instruction:");
            for i in decoded_instructions.len() - 10 .. decoded_instructions.len() {
                println!("{:?}", decoded_instructions[i]);
            }
            panic!();
        }
    }
}


