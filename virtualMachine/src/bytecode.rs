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


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn write_read_roundtrip(){
        let code = vec![0x01, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF];
        let bytes = write_program(&code);
        let decoded = read_program(&bytes).unwrap();
        assert_eq!(code, decoded);  
    }

    #[test]
    fn read_too_short(){
        let err = read_program(&[0; 8]).unwrap_err();
        assert_eq!(err, "file too short");
    }

    #[test]
    fn read_bad_magic(){
        let mut bytes = vec![0; 9];
        bytes[0] = 0x41;
        let err = read_program(&bytes).unwrap_err();
        assert_eq!(err, "invalid magic");
    }

    #[test]
    fn read_bad_version(){
        let bytes = vec![0x4D, 0x56, 0x4D, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00];
        let err = read_program(&bytes).unwrap_err();
        assert_eq!(err, "unsupported version");
    }

    #[test]
    fn read_length_mismatch(){
        let mut bytes = vec![0x4D, 0x56, 0x4D, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00];
        bytes.extend_from_slice(&[0xFF; 10]); 
        let err = read_program(&bytes).unwrap_err();
        assert_eq!(err, "length mismatch");
    }

}