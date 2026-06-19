pub const MAGIC: [u8; 4] = [0x4D, 0x56, 0x4D, 0x00];
pub const VERSION: u8 = 0x01;

pub fn write_program(code: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&MAGIC);

    bytes.push(VERSION);

    let len = code.len() as u32;
    bytes.extend_from_slice(&len.to_le_bytes());

    bytes.extend_from_slice(code);

    bytes
}

pub fn read_program(bytes: &[u8])-> Result<Vec<u8>,String>{
    if bytes.len() < 9 {
        return Err("file too short".to_string());
    }
    if bytes[0..4] != MAGIC {
        return Err("invalid magic".to_string());
    }

    if bytes[4] != VERSION {
        return Err("unsupported version".to_string());
    }

    let len = u32::from_le_bytes([
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
    ]) as usize;

    if bytes.len() != 9 + len {
        return Err("length mismatch".to_string());
    }

    Ok(bytes[9..].to_vec())
}