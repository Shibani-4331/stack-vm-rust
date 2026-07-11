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

            Op::Eq => {
                output.push_str("EQ\n");
            }
            Op::Lt => {
                output.push_str("LT\n");
            }
            Op::Gt => {
                output.push_str("GT\n");
            }
        }
        pc+=size;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disassemble_empty(){
        let text = disassemble(&[]).unwrap();
        assert_eq!(text, ""); 
    }

    #[test]
    fn disassemble_instructions(){
        let mut code = Vec::new();
        Op::Push(42).encode(&mut code);
        Op::Pop.encode(&mut code);
        Op::Dup.encode(&mut code);
        Op::Swap.encode(&mut code);
        Op::Add.encode(&mut code);
        Op::Sub.encode(&mut code);
        Op::Mul.encode(&mut code);
        Op::Div.encode(&mut code);
        Op::Mod.encode(&mut code);
        Op::Neg.encode(&mut code);
        Op::Load(0).encode(&mut code);
        Op::Store(1).encode(&mut code);
        Op::Print.encode(&mut code);
        Op::Eq.encode(&mut code);
        Op::Lt.encode(&mut code);
        Op::Gt.encode(&mut code);
        Op::Halt.encode(&mut code);
        let text = disassemble(&code).unwrap();
        let expected ="PUSH 42\nPOP\nDUP\nSWAP\nADD\nSUB\nMUL\nDIV\nMOD\nNEG\nLOAD 0\nSTORE 1\nPRINT\nEQ\nLT\nGT\nHALT\n";
        assert_eq!(text,expected);
    }

    #[test]
    fn disassemble_invalid() {
        assert!(disassemble(&[0x00]).is_err());
        assert!(disassemble(&[0x42]).is_err());
    }
}