use crate::isa::{Op, DecodeError};
use std::io::Write;
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

    pub fn run(&mut self, code: &[u8], trace: bool, step: bool) -> Result<(), VmError>  {
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

                Op::Eq => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    let c = if a == b { 1 } else { 0 };
                    self.push(c, current_ip)?;
                }
                Op::Lt => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    let c = if a < b { 1 } else { 0 };
                    self.push(c, current_ip)?;
                }
                Op::Gt => {
                    let b = self.pop(current_ip)?;
                    let a = self.pop(current_ip)?;
                    let c = if a > b { 1 } else { 0 };
                    self.push(c, current_ip)?;
                }

                Op::Jmp(addr)=>{
                    self.ip = addr as usize;
                }
                Op::Jz(addr) => {
                    let value = self.pop(current_ip)?;
                    if value == 0 {
                        self.ip = addr as usize;
                    }
                }
                Op::Jnz(addr)=>{
                    let value = self.pop(current_ip)?;
                    if value != 0 {
                        self.ip = addr as usize;
                    }
                }

                Op::Halt => {
                    halted=true;
                    break;
                }
            }
            if step {
                print!("— press Enter → ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
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



#[cfg(test)]
mod tests {
    use super::*;

    //helper fn, so we don't have to repeat encode+run pattern each time
    fn run_ops(ops: &[Op]) -> Result<Vec<i64>, VmError> {
        let mut code = Vec::new();
        for op in ops {
            op.encode(&mut code);
        }
        let mut vm = Vm::new();
        vm.run(&code, false, false)?;
        Ok(vm.stack)
    }
    #[test]
    fn test_push_pop() {
        let mut vm = Vm::new();
        vm.push(42,0).unwrap();
        assert_eq!(vm.pop(0).unwrap(), 42);
    }

    #[test]
    fn add(){
        let stack = run_ops(&[Op::Push(2), Op::Push(3), Op::Add, Op::Halt]).unwrap();
        assert_eq!(stack, vec![5]);
    }

    #[test]
    fn sub(){
        let stack = run_ops(&[Op::Push(5), Op::Push(3), Op::Sub, Op::Halt]).unwrap();
        assert_eq!(stack, vec![2]);
    }

    #[test]
    fn mul(){
        let stack = run_ops(&[Op::Push(5), Op::Push(3), Op::Mul, Op::Halt]).unwrap();
        assert_eq!(stack, vec![15]);
    }

    #[test]
    fn div(){
        let stack = run_ops(&[Op::Push(6), Op::Push(3), Op::Div, Op::Halt]).unwrap();
        assert_eq!(stack, vec![2]);
    }

    #[test]
    //finding modulo and negating the result
    fn mod_neg(){
        let stack = run_ops(&[Op::Push(20), Op::Push(14), Op::Mod, Op::Neg, Op::Halt]).unwrap();
        assert_eq!(stack, vec![-6]);
    }

    #[test]
    fn dup(){
        let stack = run_ops(&[Op::Push(5), Op::Dup, Op::Halt]).unwrap();
        assert_eq!(stack, vec![5,5]);
    }

    #[test]
    fn swap(){
        let stack = run_ops(&[Op::Push(3), Op::Push(5), Op::Swap, Op::Halt]).unwrap();
        assert_eq!(stack, vec![5,3]);
    }

    #[test]
    fn globals(){
        let stack = run_ops(&[Op::Push(42), Op::Store(0), Op::Load(0), Op::Halt]).unwrap();
        assert_eq!(stack, vec![42]);
    }
    #[test]
    fn stack_underflow() {
        let mut code = Vec::new();
        Op::Pop.encode(&mut code);
        Op::Halt.encode(&mut code);
        let mut vm = Vm::new();
        let err = vm.run(&code, false, false).unwrap_err();
        assert!(matches!(err, VmError::StackUnderflow(0)));
    }

    #[test]
    fn stack_overflow() {
        let mut code = Vec::new();
        for _ in 0..1025 {
            Op::Push(1).encode(&mut code);
        }
        Op::Halt.encode(&mut code);
        let mut vm = Vm::new();
        let err = vm.run(&code, false, false).unwrap_err();
        assert!(matches!(err, VmError::StackOverflow(9216)));
    }

    #[test]
    fn div_by_zero() {
        let stack = run_ops(&[Op::Push(10), Op::Push(0), Op::Div, Op::Halt]);
        assert!(matches!(stack, Err(VmError::DivisionByZero(18))));
    }

    #[test]
    fn mod_by_zero() {
        let stack = run_ops(&[Op::Push(10), Op::Push(0), Op::Mod, Op::Halt]);
        assert!(matches!(stack, Err(VmError::ModuloByZero(18))));
    }

    #[test]
    fn missing_halt() {
        let mut code = Vec::new();
        Op::Push(42).encode(&mut code);
        let mut vm = Vm::new();
        let err = vm.run(&code, false, false).unwrap_err();
        assert!(matches!(err, VmError::MissingHalt(9)));
    }

    #[test]
    fn invalid_opcode() {
        let mut vm = Vm::new();
        let err = vm.run(&[0x00], false, false).unwrap_err();
        assert!(matches!(err, VmError::InvalidOpcode(0x00, 0)));
    }

    #[test]
    fn eq_true() {
        let stack = run_ops(&[Op::Push(5), Op::Push(5), Op::Eq, Op::Halt]).unwrap();
        assert_eq!(stack, vec![1]);
    }

    #[test]
    fn eq_false() {
        let stack = run_ops(&[Op::Push(5), Op::Push(3), Op::Eq, Op::Halt]).unwrap();
        assert_eq!(stack, vec![0]);
    }

    #[test]
    fn lt_true() {
        let stack = run_ops(&[Op::Push(3), Op::Push(5), Op::Lt, Op::Halt]).unwrap();
        assert_eq!(stack, vec![1]);
    }

    #[test]
    fn lt_false() {
        let stack = run_ops(&[Op::Push(5), Op::Push(3), Op::Lt, Op::Halt]).unwrap();
        assert_eq!(stack, vec![0]);
    }

    #[test]
    fn gt_true() {
        let stack = run_ops(&[Op::Push(5), Op::Push(3), Op::Gt, Op::Halt]).unwrap();
        assert_eq!(stack, vec![1]);
    }

    #[test]
    fn gt_false() {
        let stack = run_ops(&[Op::Push(3), Op::Push(5), Op::Gt, Op::Halt]).unwrap();
        assert_eq!(stack, vec![0]);
    }

    #[test]
    fn jmp() {
        // PUSH 10, JMP 23 (skip PUSH 99), PUSH 20, HALT
        let stack = run_ops(&[Op::Push(10), Op::Jmp(23), Op::Push(99), Op::Push(20), Op::Halt,]).unwrap();
        assert_eq!(stack, vec![10, 20]);
    }

    #[test]
    fn jz_taken() {
        let stack = run_ops(&[Op::Push(0), Op::Jz(23), Op::Push(99), Op::Push(20), Op::Halt]).unwrap();
        assert_eq!(stack, vec![20]);
    }

    #[test]
    fn jz_not_taken() {
        let stack = run_ops(&[Op::Push(1), Op::Jz(23), Op::Push(99), Op::Halt,]).unwrap();
        assert_eq!(stack, vec![99]);
    }

    #[test]
    fn jnz_taken() {
        let stack = run_ops(&[Op::Push(1), Op::Jnz(23), Op::Push(99), Op::Push(20), Op::Halt,]).unwrap();
        assert_eq!(stack, vec![20]);
    }

    #[test]
    fn jnz_not_taken() {
        let stack = run_ops(&[Op::Push(0), Op::Jnz(23), Op::Push(99), Op::Halt,]).unwrap();
        assert_eq!(stack, vec![99]);
    }
}