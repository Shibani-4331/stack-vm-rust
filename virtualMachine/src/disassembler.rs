use crate::isa::{Op, DecodeError};

pub fn disassemble(code: &[u8])->Result<String, DecodeError> {
    let mut output = String::new();
    let mut pc = 0;

    while pc < code.len(){
        let (op,size) = Op::decode(&code[pc..])?;
        match op {
            Op::Push(n) => {
                output.push_str(&format!("PUSH {}\n", n));
            }
            Op::Pop => {
                output.push_str("POP\n");
            }
            Op::Dup => {
                output.push_str("DUP\n");
            }
            Op::Swap => {
                output.push_str("SWAP\n");
            }

            Op::Add => {
                output.push_str("ADD\n");
            }
            Op::Sub => {
                output.push_str("SUB\n");
            }
            Op::Mul => {
                output.push_str("MUL\n");
            }
            Op::Div => {
                output.push_str("DIV\n");
            }
            Op::Mod => {
                output.push_str("MOD\n");
            }
            Op::Neg => {
                output.push_str("NEG\n");
            }

            Op::Load(slot) => {
                output.push_str(&format!("LOAD {}\n", slot));
            }
            Op::Store(slot) => {
                output.push_str(&format!("STORE {}\n", slot));
            }

            Op::Print => {
                output.push_str("PRINT\n");
            }

            Op::Halt => {
                output.push_str("HALT\n");
            }
        }
        pc+=size;
    }
    Ok(output)
}