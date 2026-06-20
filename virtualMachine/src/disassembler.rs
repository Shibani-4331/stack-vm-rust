use crate::isa::{Op, DecodeError};

pub fn disassemble(code: &[u8])->Result<String, DecodeError> {
    let mut output = String::new();
    let mut pc = 0;

    while pc < code.len(){
        let (op,size) = Op::decode(&code[pc..])?;
        output.push_str(&format!("{:?}\n",op));
        pc+=size;
    }
    Ok(output)
}