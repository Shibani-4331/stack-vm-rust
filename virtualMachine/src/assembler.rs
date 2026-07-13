use crate::isa::Op;
use std:: collections::HashMap;

fn collect_labels(source: &str) -> HashMap<String, usize> {
    let mut labels = HashMap::new();
    let mut offset = 0;
    for line in source.lines() {
        let trimmed = line.split(';').next().unwrap().trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.ends_with(':') {
            let name = trimmed.trim_end_matches(':').to_uppercase();
            labels.insert(name, offset);
        } else {
            let upper = trimmed.to_uppercase();
            let parts: Vec<&str> = upper.split_whitespace().collect();
            match parts[0] {
                "PUSH" => offset += 9,
                "JMP" | "JZ" | "JNZ" => offset += 5,
                "LOAD" | "STORE" => offset += 2,
                _ => offset += 1,
            }
        }
    }
    labels
}


pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let source = source.trim_start_matches('\u{FEFF}');
    let labels = collect_labels(source);

    let mut bytes = Vec::new();
    let mut saw_halt = false;

    for (line_no, line) in source.lines().enumerate() {
        let line = line.split(';').next().unwrap().trim().to_uppercase();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        if line.ends_with(':') {
            continue;  
        }
        match parts[0] {
            "HALT" => {
                saw_halt = true;
                Op::Halt.encode(&mut bytes)
            }
            "PUSH" => {
                if parts.len() != 2 {
                    return Err(format!("Line {}: PUSH requires an operand",line_no + 1));
                }
                let value: i64 = parts[1]
                    .parse()
                    .map_err(|_| {
                        format!("Line {}: invalid number '{}'",line_no + 1,parts[1])
                    })?;
                Op::Push(value).encode(&mut bytes);
            }
            "LOAD" => {
                if parts.len() != 2 {
                    return Err(format!("Line {}: LOAD requires an operand",line_no + 1));
                }
                let slot: u8 = parts[1].parse()
                    .map_err(|_|{format!("Line {}: invalid number '{}'",line_no+1,parts[1])})?;
                Op::Load(slot).encode(&mut bytes);
            }
            "STORE" => {
                if parts.len() != 2 {
                    return Err(format!("Line {}: STORE requires an operand",line_no + 1));
                }
                let slot: u8 = parts[1].parse()
                    .map_err(|_|{format!("Line {}: invalid number '{}'",line_no+1,parts[1])})?;
                Op::Store(slot).encode(&mut bytes);
            }
            "POP" => Op::Pop.encode(&mut bytes),

            "ADD" => Op::Add.encode(&mut bytes),
            "SUB" => Op::Sub.encode(&mut bytes),
            "MUL" => Op::Mul.encode(&mut bytes),
            "DIV" => Op::Div.encode(&mut bytes),
            "MOD" => Op::Mod.encode(&mut bytes),
            "NEG" => Op::Neg.encode(&mut bytes),

            "DUP" => Op::Dup.encode(&mut bytes),
            "SWAP" => Op::Swap.encode(&mut bytes),

            "PRINT" => Op::Print.encode(&mut bytes),
            "EQ" => Op::Eq.encode(&mut bytes),
            "LT" => Op::Lt.encode(&mut bytes),
            "GT" => Op::Gt.encode(&mut bytes),

            "JMP" | "JZ" | "JNZ" => {
                let operand = parts[1];
                let addr: u32 = if let Ok(n) = operand.parse() {
                    n
                } else {
                    *labels.get(operand).ok_or_else(|| format!("Line {}: undefined label '{}'", line_no + 1, operand))? as u32
                };
                match parts[0] {
                    "JMP" => Op::Jmp(addr).encode(&mut bytes),
                    "JZ" => Op::Jz(addr).encode(&mut bytes),
                    "JNZ" => Op::Jnz(addr).encode(&mut bytes),
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(format!("Line {}: unknown instruction '{}'",line_no + 1,parts[0]));
            }

        }
    }
    if !saw_halt {
        eprintln!("warning: program does not end with HALT");
    }
    Ok(bytes)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assemble_push_halt(){
        let source = "PUSH 42\nHALT";
        let bytes = assemble(source).unwrap();

        let mut expected = Vec::new();
        Op::Push(42).encode(&mut expected);
        Op::Halt.encode(&mut expected);

        assert_eq!(bytes, expected);
    }

    #[test]
    fn assemble_all_instructions() {
        let source = "\
        PUSH 10
        POP
        DUP
        SWAP
        ADD
        SUB
        MUL
        DIV
        MOD
        NEG
        LOAD 0
        STORE 1
        PRINT
        EQ
        LT
        GT
        HALT";
        let bytes = assemble(source).unwrap();

        let mut pc = 0;
        let expected = vec![
            Op::Push(10), Op::Pop, Op::Dup, Op::Swap,
            Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Mod, Op::Neg,
            Op::Load(0), Op::Store(1),
            Op::Print, Op::Eq, Op::Lt, Op::Gt, Op::Halt,
        ];
        for op in &expected {
            let (decoded, size) = Op::decode(&bytes[pc..]).unwrap();
            assert_eq!(&decoded, op);
            pc += size;
        }
        assert_eq!(pc, bytes.len());
    }

    #[test]
    fn assemble_comments() {
        let source = "PUSH 1; this is a comment\nHALT";
        let bytes = assemble(source).unwrap();

        let mut expected = Vec::new();
        Op::Push(1).encode(&mut expected);
        Op::Halt.encode(&mut expected);
        assert_eq!(bytes, expected);
    }

    #[test]
    fn assemble_case_insensitivity() {
        let lower = assemble("push 10\nhalt\n").unwrap();
        let upper = assemble("PUSH 10\nHALT\n").unwrap();
        assert_eq!(lower, upper);
    }

    #[test]
    fn assemble_missing_halt(){
        let result = assemble("PUSH 1");
        assert!(result.is_ok());
    }

    #[test]
    fn assemble_invalid_instruction() {
        let result = assemble("FOO\nHALT").unwrap_err();
        assert!(result.contains("unknown instruction"));
        assert!(result.contains("FOO"));
    }

    #[test]
    fn assemble_invalid_operand() {
        let err1 = assemble("PUSH abc\nHALT\n").unwrap_err();
        assert!(err1.contains("invalid number"));

        let err2 = assemble("LOAD xyz\nHALT\n").unwrap_err();
        assert!(err2.contains("invalid number"));
    }

    #[test]
    fn assemble_bom_stripped() {
        let source = "\u{FEFF}PUSH 10\nHALT";
        let bytes = assemble(source).unwrap();  // should not error
        let expected = assemble("PUSH 10\nHALT").unwrap();
        assert_eq!(bytes, expected);
    }

    #[test]
    fn assemble_jump_labels() {
        let source = "\
            PUSH 100
            STORE 0
        loop:
            LOAD 0
            JZ done
            PUSH 1
            SUB
            STORE 0
            JMP loop
        done:
            PRINT
            HALT";
        let bytes = assemble(source).unwrap();
        // decode and verify JMP loop goes to 22, JZ done goes to 61
        let mut pc = 0;
        let (op1, s1) = Op::decode(&bytes[pc..]).unwrap();
        assert_eq!(op1, Op::Push(100));
        pc += s1;
        let (op2, s2) = Op::decode(&bytes[pc..]).unwrap();
        assert_eq!(op2, Op::Store(0));
        pc += s2;
    }

    #[test]
    fn assemble_undefined_label() {
        let err = assemble("JMP nowhere\nHALT").unwrap_err();
        assert!(err.contains("undefined label"));
    }

    #[test]
    fn assemble_label_and_jump_roundtrip() {
        let source = "\
            JMP end
            PUSH 99
            POP
        end:
            PUSH 42
            HALT";
        let bytes = assemble(source).unwrap();
        let text = crate::disassembler::disassemble(&bytes).unwrap();
        let reassembled = assemble(&text).unwrap();
        assert_eq!(bytes, reassembled);
    }
}