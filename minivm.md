> **Build a stack-based bytecode VM** with its own assembler and disassembler. Write programs in your own assembly language, compile them to a binary format you defined, and execute them on a machine you built.

<span style="background:#7c2d12;color:#fff;padding:2px 8px;border-radius:6px;font-size:12px">Intermediate</span>

| | |
|---|---|
| **Stack** | `std` only, zero external crates |
| **Deliverable** | GitHub repo + README with infix-to-stack-code translation table |
| **Pipeline** | `program.tasm` -> [ assembler ] -> `program.tbc` -> [ VM ] -> output |
| **Due** | end of course |

### What you'll build

The back half of a programming language implementation: the part after parsing. One binary, three subcommands sharing a single instruction definition. `minivm asm` compiles assembly text to bytecode, `minivm run` executes it on a stack machine, `minivm dis` turns bytecode back into text. The ISA is straight-line arithmetic only: no jumps, no loops. Every program runs top to bottom, once. This is the same architecture as Python's `.pyc` files and the JVM, stripped to the bone.

### Core requirements

- [ ] `minivm asm <file.tasm> -o <file.tbc>` - single-pass assembler with line-numbered errors
- [ ] `minivm run <file.tbc>` - stack machine executing the frozen ISA below
- [ ] `minivm run --trace` - prints ip, instruction, and stack before every step
- [ ] `minivm dis <file.tbc>` - disassembler; `asm -> dis -> asm` must be byte-identical
- [ ] All 5 trap classes detected and reported with the offending `ip`, exit nonzero
- [ ] Bytecode files validated: magic, version, length header

### The frozen ISA (implement exactly, change nothing)

64-bit signed integers. Operand stack (max 1024). 256 global `i64` slots, zero-initialized. `ip` only moves forward. Multi-byte operands are little-endian.

| Byte | Mnemonic | Operand | Effect |
|------|----------|---------|--------|
| 0x01 | `PUSH n` | i64 | Push `n` |
| 0x02 | `POP` | - | Discard top |
| 0x03 | `DUP` | - | Duplicate top |
| 0x04 | `SWAP` | - | Swap top two |
| 0x10 | `ADD` | - | Pop b, pop a, push a+b |
| 0x11 | `SUB` | - | Pop b, pop a, push a-b |
| 0x12 | `MUL` | - | Pop b, pop a, push a*b |
| 0x13 | `DIV` | - | Pop b, pop a, push a/b (trap if b=0) |
| 0x14 | `MOD` | - | Pop b, pop a, push a%b (trap if b=0) |
| 0x15 | `NEG` | - | Pop a, push -a |
| 0x40 | `LOAD s` | u8 slot | Push global slot `s` |
| 0x41 | `STORE s` | u8 slot | Pop into global slot `s` |
| 0x60 | `PRINT` | - | Pop, print with newline |
| 0xFF | `HALT` | - | Stop |

**Traps:** stack overflow/underflow, div/mod by zero, unknown opcode, truncated instruction, `ip` past end without `HALT`. Format: `trap at ip=0x002A: stack underflow (POP on empty stack)`

**File format:** magic `0x4D 0x56 0x4D 0x00` ("MVM\0"), version `0x01`, code length (u32 LE), then raw code.

**Assembly syntax:** one instruction per line, `;` comments, case-insensitive mnemonics, must end in `HALT` (warn otherwise).

### Starter

```rust
// isa.rs is the ONLY file that knows byte encodings.
// asm, dis, and vm all go through it. If you write 0x41
// in two places, you have made a mistake.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Push(i64), Pop, Dup, Swap,
    Add, Sub, Mul, Div, Mod, Neg,
    Load(u8), Store(u8),
    Print, Halt,
}
```

```asm
; (7 + 3) * (9 - 4) / 5 - comments show the stack AFTER each line
        PUSH 7          ; [7]
        PUSH 3          ; [7, 3]
        ADD             ; [10]        pops 3 and 7, pushes their sum
        PUSH 9          ; [10, 9]
        PUSH 4          ; [10, 9, 4]
        SUB             ; [10, 5]
        MUL             ; [50]
        PUSH 5          ; [50, 5]
        DIV             ; [10]
        PRINT           ; []
        HALT
```

### Acceptance tests

- [ ] `arith.tasm` - `(7 + 3) * (9 - 4) / 5` prints `10`
- [ ] `horner.tasm` - `3x^3 + 2x^2 + 5x + 7` at `x = 11` via Horner's method, `x` in a global slot, prints `4297`
- [ ] `celsius.tasm` - 100 C to Fahrenheit, integer ops only, prints `212`
- [ ] `stackplay.tasm` - `a^2 + b^2` for a=12, b=35, each input pushed exactly once (forces `DUP`/`SWAP`/slots), prints `1369`
- [ ] `digits.tasm` - prints the digits of 9274 on four lines using only `DIV`/`MOD`, unrolled
- [ ] One `.tasm` per trap class, each reporting the correct trap and ip
- [ ] `asm -> dis -> asm` byte-identical for every test program
- [ ] README: infix expression -> stack code translation table for each program

### Stretch goals

- [ ] Comparisons: `EQ` 0x20, `LT` 0x21, `GT` 0x22
- [ ] Jumps: `JMP` 0x30, `JZ` 0x31, `JNZ` 0x32 (u32 offsets) - forces labels + a two-pass assembler. Prove it with a loop printing sum of 1..=100 (`5050`)
- [ ] `--step` mode: one instruction per Enter keypress

> 💡 **Tip:** build `--trace` in the first days, not the last. It is your debugger for everything that follows. And write the encode/decode round-trip test for `isa.rs` before the assembler exists.

### Resources

**Concepts**
- [Stack Machines: Fundamentals](https://igor.io/2013/08/28/stack-machines-fundamentals.html) - a short blog series that builds exactly this project step by step; read at least the first two posts ([RPN calculator](https://igor.io/2013/12/02/stack-machines-rpn) is post two). Posts three onward cover jumps, calls, and stack frames - that is your stretch-goal reading
- [Crafting Interpreters, ch. 14: Chunks of Bytecode](https://craftinginterpreters.com/chunks-of-bytecode.html) - the canonical walkthrough of designing a bytecode format and the why behind every decision you are about to make
- [Understanding Big and Little Endian Byte Order](https://betterexplained.com/articles/understanding-big-and-little-endian-byte-order/) - why the spec says little-endian and what it means for your encode/decode
- [nand2tetris, projects 7-8](https://www.nand2tetris.org/) - a famous course whose VM translator is this same machine built from the hardware side up

**The real thing**
- [`python -m dis`](https://docs.python.org/3/library/dis.html) - disassemble any Python function and recognize your own `minivm dis` output; CPython is this project with more opcodes
- [A Python Interpreter Written in Python](https://aosabook.org/en/500L/a-python-interpreter-written-in-python.html) - CPython's VM rebuilt in ~500 readable lines
