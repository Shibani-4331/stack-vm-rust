use crate::isa::{Op, DecodeError};
pub struct Vm {
    stack : Vec<i64>,
    globals: [i64;256],
    ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: [0; 256], // globals are zero-initialized
            ip: 0,
        }
    }
    pub fn push(&mut self, value: i64, ip:usize) -> Result<(), VmError> {
        if self.stack.len() >= 1024 {
            return Err(VmError::StackOverflow(ip)); // temporary
        }
        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self, ip:usize) -> Result<i64, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow(ip))
    }

    pub fn run(&mut self, code: &[u8], trace: bool) -> Result<(), VmError> {
        self.ip = 0;
        let mut halted = false;
        while self.ip < code.len() {
            let current_ip = self.ip;
            let (op, size) = Op::decode(&code[self.ip..])
            .map_err(|e| match e {
                DecodeError::InvalidOpcode(op) => VmError::InvalidOpcode(op, current_ip),
                DecodeError::TruncatedInstruction => VmError::TruncatedInstruction(current_ip),
            })?;
            if trace {
                println!(
                    "ip={} op={:?} stack={:?}",
                    current_ip,
                    op,
                    self.stack
                );
            }
            self.ip += size;
            match op {
                Op::Push(n) => {
                    self.push(n,current_ip)?;
                }
                Op::Pop => {
                    self.pop(current_ip)?;
                }

                Op::Add => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    self.push(a + b,current_ip)?;
                }
                Op::Sub => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    self.push(a - b,current_ip)?;
                }
                Op::Mul => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    self.push(a*b, current_ip)?;
                }
                Op::Div => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    if b == 0 {
                        return Err(VmError::DivisionByZero(current_ip));
                    }
                    self.push(a / b, current_ip)?;
                }
                Op::Neg => {
                    let a = self.pop(current_ip)?;
                    self.push(-a, current_ip)?;
                }
                Op::Mod => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;

                    if b == 0 {
                        return Err(VmError::ModuloByZero(current_ip));
                    }
                    self.push(a % b, current_ip)?;
                }
                Op::Dup => {
                    let m = *self.stack.last().ok_or(VmError::StackUnderflow(current_ip))?;
                    self.push(m,current_ip)?;
                }
                Op::Swap =>{
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    self.push(b,current_ip)?;
                    self.push(a,current_ip)?;
                }
                
                Op::Store(slot) => {
                    let value = self.pop(current_ip)?;
                    self.globals[slot as usize] = value;
                }
                Op::Load(slot) => {
                    let value = self.globals[slot as usize];
                    self.push(value,current_ip)?;
                }
                Op::Print => {
                    let value = self.pop(current_ip)?;
                    println!("{}", value);
                }
                Op::Halt => {
                    halted=true;
                    break;
                }
            }
        }
        if !halted {
            return Err(VmError::MissingHalt(self.ip));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum VmError {
    StackUnderflow(usize),
    StackOverflow(usize),
    DivisionByZero(usize),
    ModuloByZero(usize),
    MissingHalt(usize),
    InvalidOpcode(u8, usize),
    TruncatedInstruction(usize),
}
impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmError::StackUnderflow(ip) =>
                write!(f, "trap at ip=0x{:04X}: stack underflow", ip),

            VmError::StackOverflow(ip) =>
                write!(f, "trap at ip=0x{:04X}: stack overflow", ip),

            VmError::DivisionByZero(ip) =>
                write!(f, "trap at ip=0x{:04X}: division by zero", ip),

            VmError::ModuloByZero(ip) =>
                write!(f, "trap at ip=0x{:04X}: modulo by zero", ip),

            VmError::MissingHalt(ip) =>
                write!(f, "trap at ip=0x{:04X}: program ended without HALT", ip),

            VmError::TruncatedInstruction(ip) =>
                write!(f, "trap at ip=0x{:04X}: truncated instruction", ip),

            VmError::InvalidOpcode(op, ip) =>
                write!(f,
                    "trap at ip=0x{:04X}: invalid opcode 0x{:02X}",ip , op),
        }
    }
}
