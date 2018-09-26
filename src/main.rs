mod cpu;

use std::io;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::ops::Add;

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

    let results = decode_instructions_from_file(reader);
    println!("Decoded {} instructions.", results.len());
    let output_file_path = Path::new(&out_path_name);
    let mut out_file = File::create(output_file_path).expect("Unable to write output file, aborting.");

    write_output_to_file(results, BufWriter::new(out_file));
}

pub fn decode_instructions_from_file(mut reader: BufReader<File>) -> Vec<Instruction>
{
    let mut opcode_buffer : Vec<u8> = vec!();
    println!("Reading file into system!");
    reader.read_to_end(&mut opcode_buffer).expect("Unable to read from file. Aborting.");
    let mut opcodes = VecDeque::from_iter(opcode_buffer.into_iter());
    println!("Successfully read {} instructions, decoding..", opcodes.len());
    cpu::decode_instructions(opcodes)
}


pub fn write_output_to_file(results: Vec<Instruction>, mut out: BufWriter<File>)
{
    let mut output_buf = String::new();
    for i in 0..results.len() {
        output_buf = output_buf.add(
            format!("{:#00005x}    ", i).as_str()
        )
        .add(
            format!("{:?}", results[i]).as_str()
        )
        .add("\n");
    }
    out.write_all(output_buf.as_bytes());
}