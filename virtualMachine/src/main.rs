mod isa;
mod vm;
mod assembler;
mod bytecode;
mod disassembler;

use isa::Op;
use vm::Vm;
use bytecode::{write_program, read_program};
use std::fs;
use crate::assembler::assemble;
use disassembler::disassemble;

fn main() {
    
    // let mut bytes = Vec::new();

    // // STORE / LOAD
    // Op::Push(42).encode(&mut bytes);
    // Op::Store(0).encode(&mut bytes);
    // Op::Load(0).encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // 42

    // // ADD
    // Op::Push(10).encode(&mut bytes);
    // Op::Push(20).encode(&mut bytes);
    // Op::Add.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // 30

    // // SUB
    // Op::Push(10).encode(&mut bytes);
    // Op::Push(3).encode(&mut bytes);
    // Op::Sub.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // 7

    // // MUL
    // Op::Push(6).encode(&mut bytes);
    // Op::Push(7).encode(&mut bytes);
    // Op::Mul.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // 42

    // // DIV
    // Op::Push(50).encode(&mut bytes);
    // Op::Push(5).encode(&mut bytes);
    // Op::Div.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // 10

    // // MOD
    // Op::Push(17).encode(&mut bytes);
    // Op::Push(5).encode(&mut bytes);
    // Op::Mod.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // 2

    // // NEG
    // Op::Push(7).encode(&mut bytes);
    // Op::Neg.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // -7

    // // DUP
    // Op::Push(9).encode(&mut bytes);
    // Op::Dup.encode(&mut bytes);
    // Op::Add.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // 18

    // // SWAP
    // Op::Push(10).encode(&mut bytes);
    // Op::Push(3).encode(&mut bytes);
    // Op::Swap.encode(&mut bytes);
    // Op::Sub.encode(&mut bytes);
    // Op::Print.encode(&mut bytes);      // -7

    // Op::Halt.encode(&mut bytes);

    // let mut vm = Vm::new();
    // let src = "
    // PUSH 42
    // STORE 0
    // LOAD 0
    // PRINT
    // HALT
    // ";
    // let bytes = assembler::assemble(src).unwrap();
    // println!("{:?}", bytes);

    // let code = vec![0xFF];
    // let file = write_program(&code);
    // println!("{:?}", file);

    // let code = vec![1, 2, 3, 255];
    // let file = write_program(&code);
    // let recovered = read_program(&file).unwrap();
    // println!("{:?}", recovered);

    // let source = fs::read_to_string("program.tasm").unwrap();
    // let code = assemble(&source).unwrap();
    // let file_bytes = write_program(&code);
    // fs::write("program.tbc", &file_bytes).unwrap();

    // let file_bytes = fs::read("program.tbc").unwrap();
    // let code = read_program(&file_bytes).unwrap();
    // let mut vm = Vm::new();
    // vm.run(&code, false).unwrap();

    // let text = disassemble(&code).unwrap();
    // println!("{}",text);

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  vm asm <file.tasm>");
        eprintln!("  vm run <file.tbc>");
        eprintln!("  vm dis <file.tbc>");
        eprintln!(" vm trace <file.tbc>");
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "asm" => {
            let source = std::fs::read_to_string(filename)
                .expect("Failed to read source file");

            let code = assemble(&source)
                .expect("Assembly failed");

            let file_bytes = write_program(&code);

            std::fs::write("program.tbc", file_bytes)
                .expect("Failed to write bytecode file");

            println!("Assembled successfully.");
        }

        "run" => {
            let file_bytes = std::fs::read(filename)
                .expect("Failed to read bytecode file");

            let code = read_program(&file_bytes)
                .expect("Invalid bytecode file");

            let mut vm = Vm::new();

            vm.run(&code, false)
                .expect("VM execution failed");
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

            vm.run(&code, true)
                .expect("VM execution failed");
        }
        _ => {
            eprintln!("Unknown command");
        }
    }
}
