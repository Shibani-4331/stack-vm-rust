mod isa;
mod vm;

use isa::Op;
use vm::Vm;

fn main() {
    let mut bytes = Vec::new();

    // STORE / LOAD
    Op::Push(42).encode(&mut bytes);
    Op::Store(0).encode(&mut bytes);
    Op::Load(0).encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // 42

    // ADD
    Op::Push(10).encode(&mut bytes);
    Op::Push(20).encode(&mut bytes);
    Op::Add.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // 30

    // SUB
    Op::Push(10).encode(&mut bytes);
    Op::Push(3).encode(&mut bytes);
    Op::Sub.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // 7

    // MUL
    Op::Push(6).encode(&mut bytes);
    Op::Push(7).encode(&mut bytes);
    Op::Mul.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // 42

    // DIV
    Op::Push(50).encode(&mut bytes);
    Op::Push(5).encode(&mut bytes);
    Op::Div.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // 10

    // MOD
    Op::Push(17).encode(&mut bytes);
    Op::Push(5).encode(&mut bytes);
    Op::Mod.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // 2

    // NEG
    Op::Push(7).encode(&mut bytes);
    Op::Neg.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // -7

    // DUP
    Op::Push(9).encode(&mut bytes);
    Op::Dup.encode(&mut bytes);
    Op::Add.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // 18

    // SWAP
    Op::Push(10).encode(&mut bytes);
    Op::Push(3).encode(&mut bytes);
    Op::Swap.encode(&mut bytes);
    Op::Sub.encode(&mut bytes);
    Op::Print.encode(&mut bytes);      // -7

    Op::Halt.encode(&mut bytes);

    let mut vm = Vm::new();
}
