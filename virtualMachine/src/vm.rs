use crate::isa::Op;


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

    pub fn push(&mut self, value:i64){
        self.stack.push(value);
    }

    pub fn pop(&mut self)->i64{
        self.stack.pop().unwrap()
    }

    pub fn run(&mut self, code:&[u8]){
        while self.ip < code.len() {
            let (op, size) = Op::decode(&code[self.ip..]).unwrap();
            self.ip += size;
            match op {
                Op::Push(n) => {
                    self.push(n);
                }
                Op::Pop => {
                    self.pop();
                }

                Op::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a + b);
                }
                Op::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a - b);
                }
                Op::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a*b);
                }
                Op::Div => {
                    let b = self.pop();
                    let a = self.pop();

                    self.push(a / b);
                }
                Op::Neg => {
                    let a = self.pop();
                    self.push(-a);
                }
                Op::Mod => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(a % b);
                }

                Op::Dup => {
                    let m:i64 = *self.stack.last().unwrap();
                    self.push(m);
                }
                Op::Swap =>{
                    let b = self.pop();
                    let a = self.pop();
                    self.push(b);
                    self.push(a);
                }
                
                Op::Store(slot) => {
                    let value = self.pop();
                    self.globals[slot as usize] = value;
                }
                Op::Load(slot) => {
                    let value = self.globals[slot as usize];
                    self.push(value);
                }
                Op::Print => {
                    let value = self.pop();
                    println!("{}", value);
                }
                Op::Halt => {
                    break;
                }

                _ => todo!(),
            }
        }
    }
}