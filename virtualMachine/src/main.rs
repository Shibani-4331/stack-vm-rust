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
        eprintln!("  vm run [--trace] [--step] <file.tbc>");
        eprintln!("  vm dis <file.tbc>");
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "asm" => {
            if args.len() < 5 || args[3] != "-o" {
                eprintln!("Usage: vm asm <file.tasm> -o <file.tbc>");
                std::process::exit(1);
            }

            let output_file = &args[4];

            let source = std::fs::read_to_string(filename)
                .unwrap_or_else(|e| { 
                    eprintln!("Error: {}", e); 
                    std::process::exit(1); 
                });
            let code = assemble(&source)
                .unwrap_or_else(|e| { 
                    eprintln!("{}", e); 
                    std::process::exit(1); 
                });

            let file_bytes = write_program(&code);

            std::fs::write(output_file, file_bytes)
                .unwrap_or_else(|e| { 
                    eprintln!("Error: {}", e); 
                    std::process::exit(1); 
                });

            println!("Assembled successfully.");
        }

        "run" => {
            let mut trace = false;
            let mut step = false;
            let mut file_idx = 2;
            while file_idx < args.len() && args[file_idx].starts_with("--") {
                match args[file_idx].as_str() {
                    "--trace" => trace = true,
                    "--step" => step = true,
                    _ => { eprintln!("Unknown flag '{}'", args[file_idx]); std::process::exit(1); }
                }
                file_idx += 1;
            }
            if step && !trace {
                trace = true;
            }
            if file_idx >= args.len() {
                eprintln!("Usage: vm run [--trace] [--step] <file.tbc>");
                std::process::exit(1);
            }
            let file = &args[file_idx];

            let file_bytes = std::fs::read(file)
                .unwrap_or_else(|e| { 
                    eprintln!("Error: {}", e); 
                    std::process::exit(1); 
                });

            let code = match read_program(&file_bytes) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
            };
            let mut vm = Vm::new();

            if let Err(err) = vm.run(&code, trace, step) {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }

        "dis" => {
            let file_bytes = std::fs::read(filename)
                .unwrap_or_else(|e| { 
                    eprintln!("Error: {}", e); 
                    std::process::exit(1); 
                });

            let code = match read_program(&file_bytes) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                }
            };
            let text = disassemble(&code)
                .expect("Disassembly failed");

            println!("{}", text);
        }
        _ => {
            eprintln!("Unknown command");
            std::process::exit(1);
        }
    }
}
