#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Push(i64),
    Pop,
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Load(u8),
    Store(u8),
    Print,
    Halt,
}
 impl Op{
    pub fn encode(&self, out:&mut Vec<u8>){
        match self{
            Op::Push(n)=>{
                out.push(0x01);
                out.extend_from_slice(&n.to_le_bytes());
            }
            Op::Pop => out.push(0x02),
            Op::Dup => out.push(0x03),
            Op::Swap => out.push(0x04),
            Op::Add => out.push(0x10),
            Op::Sub => out.push(0x11),
            Op::Mul => out.push(0x12),
            Op::Div => out.push(0x13),
            Op::Mod => out.push(0x14),
            Op::Neg => out.push(0x15),
            Op::Load(slot) => {
                out.push(0x40);
                out.push(*slot);
            }
            Op::Store(val) => {
                out.push(0x41);
                out.push(*val);
            }
            Op::Print => out.push(0x60),
            Op::Halt => out.push(0xFF),
        }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Op, usize), DecodeError> {
        if bytes.is_empty() {
            return Err(DecodeError::TruncatedInstruction);
        }

        match bytes[0] {
            0x02 => Ok((Op::Pop, 1)),
            0x03 => Ok((Op::Dup, 1)),
            0x04 => Ok((Op::Swap, 1)),

            0x10 => Ok((Op::Add, 1)),
            0x11 => Ok((Op::Sub, 1)),
            0x12 => Ok((Op::Mul, 1)),
            0x13 => Ok((Op::Div, 1)),
            0x14 => Ok((Op::Mod, 1)),
            0x15 => Ok((Op::Neg, 1)),

            0x60 => Ok((Op::Print, 1)),
            0xFF => Ok((Op::Halt, 1)),

            _=>todo!()
        }
    }
 }

#[derive(Debug)]
pub enum DecodeError {
    UnknownOpcode(u8),
    TruncatedInstruction,
}
