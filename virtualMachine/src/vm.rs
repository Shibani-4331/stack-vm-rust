// use std::fmt::write;

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
    pub fn push(&mut self, value:i64)->Result<(), VmError>{
        if self.stack.len() >= 1024{
           return Err(VmError::StackOverflow);
        }
        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self)-> Result<i64, VmError>{
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }

    pub fn run(&mut self, code: &[u8], trace: bool) -> Result<(), VmError> {
        let mut halted = false;
        while self.ip < code.len() {
            let current_ip = self.ip;
            let (op, size) = Op::decode(&code[self.ip..])
            .map_err(|e| match e {
                DecodeError::InvalidOpcode(op) => VmError::InvalidOpcode(op),
                DecodeError::TruncatedInstruction => VmError::TruncatedInstruction,
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
                    self.push(n)?;
                }
                Op::Pop => {
                    self.pop()?;
                }

                Op::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a + b)?;
                }
                Op::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a - b)?;
                }
                Op::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a*b)?;
                }
                Op::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err(VmError::DivisionByZero);
                    }
                    self.push(a / b)?;
                }
                Op::Neg => {
                    let a = self.pop()?;
                    self.push(-a)?;
                }
                Op::Mod => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    if b == 0 {
                        return Err(VmError::ModuloByZero);
                    }
                    self.push(a % b)?;
                }
                Op::Dup => {
                    let m = *self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.push(m)?;
                }
                Op::Swap =>{
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(b)?;
                    self.push(a)?;
                }
                
                Op::Store(slot) => {
                    let value = self.pop()?;
                    self.globals[slot as usize] = value;
                }
                Op::Load(slot) => {
                    let value = self.globals[slot as usize];
                    self.push(value)?;
                }
                Op::Print => {
                    let value = self.pop()?;
                    println!("{}", value);
                }
                Op::Halt => {
                    halted=true;
                    break;
                }
            }
        }
        if !halted {
            return Err(VmError::MissingHalt);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum VmError {
    StackUnderflow,
    StackOverflow,
    DivisionByZero,
    ModuloByZero,
    MissingHalt,
    InvalidOpcode(u8),
    TruncatedInstruction,
}
impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmError::StackUnderflow => write!(f, "stack underflow"),
            VmError::StackOverflow => write!(f, "stack overflow"),
            VmError::DivisionByZero => write!(f, "division by zero"),
            VmError::ModuloByZero => write!(f, "modulo by zero"),
            VmError::MissingHalt =>write!(f,"program ended without HALT"),
            VmError::TruncatedInstruction=>write!(f,"truncated instruction"),
            VmError::InvalidOpcode(op)=>write!(f,"invalid opcode: 0x{:02X}",op),
        }
    }
}
