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
    Eq,
    Lt,
    Gt,
    Jmp(u32),
    Jz(u32),
    Jnz(u32),
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
            Op::Eq => out.push(0x20),
            Op::Lt => out.push(0x21),
            Op::Gt => out.push(0x22),
            Op::Jmp(addr) => {
                out.push(0x30);
                out.extend_from_slice(&addr.to_le_bytes());
            }
            Op::Jz(addr) => {
                out.push(0x31);
                out.extend_from_slice(&addr.to_le_bytes());
            }
            Op::Jnz(addr) => {
                out.push(0x32);
                out.extend_from_slice(&addr.to_le_bytes());
            }
        }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Op, usize), DecodeError> {
        if bytes.is_empty() {
            return Err(DecodeError::TruncatedInstruction);
        }

        match bytes[0] {
            0x01 => {
                if bytes.len() < 9 {
                    return Err(DecodeError::TruncatedInstruction);
                }

                let num_bytes: [u8; 8] = bytes[1..9]//converts &[u8] type to an Array of type [u8,8]
                    .try_into()
                    .unwrap();

                let value = i64::from_le_bytes(num_bytes);// convert bytes-> i64

                Ok((Op::Push(value), 9))
            }
            0x02 => Ok((Op::Pop, 1)),
            0x03 => Ok((Op::Dup, 1)),
            0x04 => Ok((Op::Swap, 1)),

            0x10 => Ok((Op::Add, 1)),
            0x11 => Ok((Op::Sub, 1)),
            0x12 => Ok((Op::Mul, 1)),
            0x13 => Ok((Op::Div, 1)),
            0x14 => Ok((Op::Mod, 1)),
            0x15 => Ok((Op::Neg, 1)),
            
            0x40 =>{
                if bytes.len()<2{
                    return Err(DecodeError::TruncatedInstruction)
                }
                Ok((Op::Load(bytes[1]),2))// bytes[1]=slot number, bytes[0]=load->64
            }
            0x41 =>{
                if bytes.len()<2{
                    return Err(DecodeError::TruncatedInstruction)
                }
                Ok((Op::Store(bytes[1]),2))// bytes[1]=slot number, bytes[0]=load->65
            }
            0x60 => Ok((Op::Print, 1)),
            0xFF => Ok((Op::Halt, 1)),
            
            0x20 => Ok((Op::Eq, 1)),
            0x21 => Ok((Op::Lt, 1)),
            0x22 => Ok((Op::Gt, 1)),

            0x30 => {
                if bytes.len() < 5 {
                    return Err(DecodeError::TruncatedInstruction);
                }
                let addr = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok((Op::Jmp(addr), 5))
            }
            0x31 => {
                if bytes.len() < 5 {
                    return Err(DecodeError::TruncatedInstruction);
                }
                let addr = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok((Op::Jz(addr), 5))
            }
            0x32 => {
                if bytes.len() < 5 {
                    return Err(DecodeError::TruncatedInstruction);
                }
                let addr = u32::from_le_bytes(bytes[1..5].try_into().unwrap());
                Ok((Op::Jnz(addr), 5))
            }
            opcode => Err(DecodeError::InvalidOpcode(opcode))
        }
    }
 }

#[derive(Debug)]
pub enum DecodeError {
    InvalidOpcode(u8),
    TruncatedInstruction,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_push(){
        let op = Op::Push(42);
        let mut bytes = Vec::new();
        op.encode(&mut bytes);
        let (decoded_op, size) = Op::decode(&bytes).unwrap();
        assert_eq!(decoded_op, op);
        assert_eq!(size, 9);
    }

    #[test]
    fn all_variants_round_trip() {
        let ops = vec![
            Op::Push(0), Op::Push(-1), Op::Push(i64::MAX), Op::Push(i64::MIN),
            Op::Pop, Op::Dup, Op::Swap,
            Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Mod, Op::Neg,
            Op::Load(0), Op::Load(255), Op::Store(0), Op::Store(255),
            Op::Print, Op::Halt,Op::Eq, Op::Lt, Op::Gt,
            Op::Jmp(0), Op::Jmp(u32::MAX), Op::Jz(0), Op::Jz(255), Op::Jnz(0xDEADBEEF),
        ];
        for op in ops {
            let mut bytes = Vec::new();
            op.encode(&mut bytes);
            let (decoded, _) = Op::decode(&bytes).unwrap();
            assert_eq!(decoded, op);
        }
    }

    #[test]
    fn decode_truncated(){
        assert!(matches!(Op::decode(&[]), Err(DecodeError::TruncatedInstruction)));
        assert!(matches!(Op::decode(&[0x01]), Err(DecodeError::TruncatedInstruction)));
        assert!(matches!(Op::decode(&[0x40]), Err(DecodeError::TruncatedInstruction)));
        assert!(matches!(Op::decode(&[0x30]), Err(DecodeError::TruncatedInstruction)));
        assert!(matches!(Op::decode(&[0x31]), Err(DecodeError::TruncatedInstruction)));
        assert!(matches!(Op::decode(&[0x32]), Err(DecodeError::TruncatedInstruction)));
    }

    #[test]
    fn decode_invalid_opcode() {
        assert!(matches!(Op::decode(&[0x00]), Err(DecodeError::InvalidOpcode(0x00))));
        assert!(matches!(Op::decode(&[0x42]), Err(DecodeError::InvalidOpcode(0x42))));
    }
}