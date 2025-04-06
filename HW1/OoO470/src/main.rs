mod activelist;
mod alu;
mod bbt;
mod cpu;
mod dir;
mod eflag;
mod freelist;
mod integerqueue;
mod op;
mod pc;
mod physregfile;
mod rmt;
use std::{fs::File, io::BufReader, path::Path};

use cpu::CPU;
use op::Instruction;
use serde_json::Value;

fn main() {
    // Create instruction
    println!("Parsing of instructions!");

    let path = Path::new("../given_tests/01/input.json");

    let instructions = parse_instructions(path);

    let mut cpu = CPU::from_instructions(instructions);

    cpu.dump_state_into_log();
}

fn parse_instructions(path: &Path) -> Vec<Instruction> {
    let file = File::open(path).unwrap();

    let reader = BufReader::new(file);

    let instructions_str: Vec<String> = serde_json::from_reader(reader).unwrap();

    let mut instructions: Vec<Instruction> = vec![];

    for instr in instructions_str {
        let i = Instruction::from_str(instr);
        instructions.push(i);
    }

    return instructions;
}
