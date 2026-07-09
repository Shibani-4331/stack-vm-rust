use crate::isa::Op;

pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    let mut saw_halt = false;
    for (line_no, line) in source.lines().enumerate() {
        let line = line.split(';').next().unwrap().trim().to_uppercase();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
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
        HALT";
        let bytes = assemble(source).unwrap();

        let mut pc = 0;
        let expected = vec![
            Op::Push(10), Op::Pop, Op::Dup, Op::Swap,
            Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Mod, Op::Neg,
            Op::Load(0), Op::Store(1),
            Op::Print, Op::Halt,
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
}