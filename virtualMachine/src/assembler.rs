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
                if parts.len() < 2 {
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
                if parts.len() < 2 {
                    return Err(format!("Line {}: LOAD requires an operand",line_no + 1));
                }
                let slot: u8 = parts[1].parse()
                    .map_err(|_|{format!("Line {}: invalid number '{}'",line_no+1,parts[1])})?;
                Op::Load(slot).encode(&mut bytes);
            }
            "STORE" => {
                if parts.len() < 2 {
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