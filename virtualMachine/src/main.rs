mod isa;
mod vm;
mod assembler;
mod bytecode;
mod disassembler;

use vm::Vm;
use bytecode::{write_program, read_program};
use crate::assembler::assemble;
use disassembler::disassemble;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  vm asm <file.tasm> -o <file.tbc>");
        eprintln!("  vm run <file.tbc>");
        eprintln!("  vm dis <file.tbc>");
        eprintln!("  vm trace <file.tbc>");
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "asm" => {
            if args.len() < 5 || args[3] != "-o" {
                eprintln!("Usage: vm asm <file.tasm> -o <file.tbc>");
                return;
            }

            let output_file = &args[4];

            let source = std::fs::read_to_string(filename)
                .expect("Failed to read source file");

            let code = assemble(&source)
                .expect("Assembly failed");

            let file_bytes = write_program(&code);

            std::fs::write(output_file, file_bytes)
                .expect("Failed to write bytecode file");

            println!("Assembled successfully.");
        }

        "run" => {
            let file_bytes = std::fs::read(filename)
                .expect("Failed to read bytecode file");

            let code = read_program(&file_bytes)
                .expect("Invalid bytecode file");

            let mut vm = Vm::new();

            if let Err(err) = vm.run(&code, false){
                eprintln!("Error: {}", err)
            }
        }

        "dis" => {
            let file_bytes = std::fs::read(filename)
                .expect("Failed to read bytecode file");

            let code = read_program(&file_bytes)
                .expect("Invalid bytecode file");

            let text = disassemble(&code)
                .expect("Disassembly failed");

            println!("{}", text);
        }
        "trace" => {
            let file_bytes = std::fs::read(filename)
                .expect("Failed to read bytecode file");

            let code = read_program(&file_bytes)
                .expect("Invalid bytecode file");

            let mut vm = Vm::new();

            if let Err(err) = vm.run(&code, true){
                eprintln!("Error: {}", err)
            }
        }
        _ => {
            eprintln!("Unknown command");
        }
    }
}
