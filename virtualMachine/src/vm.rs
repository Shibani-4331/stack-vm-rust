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
}