# Stack VM in Rust

A stack-based bytecode virtual machine with a custom assembler and disassembler, built from scratch using only the standard library.

```
program.tasm  →  [assembler]  →  program.tbc  →  [VM]  →  output
                   [disassembler]  ←──┘
```

## Quick Start

```sh
cargo run -- asm examples/arithmetic.tasm -o examples/arithmetic.tbc
cargo run -- run examples/arithmetic.tbc
# 10

cargo run -- run --trace examples/arithmetic.tbc
# ip=0 op=Push(7) stack=[]
# ip=9 op=Push(3) stack=[7]
# ...

cargo run -- run --step examples/arithmetic.tbc
# ip=0  op=Push(7)  stack=[]
# — press Enter →
# ip=9  op=Push(3)  stack=[7]
# — press Enter →
# ...

cargo run -- dis examples/arithmetic.tbc
# PUSH 7
# PUSH 3
# ADD
# ...
```

## Assembly Language (.tasm)

- One instruction per line
- `;` starts a comment (rest of line ignored)
- Mnemonics are case-insensitive
- Must end with `HALT` (a warning is emitted otherwise)
- Operands: `PUSH` takes an `i64`, `LOAD`/`STORE` take a `u8` slot number, `CALL`/`JMP`/`JZ`/`JNZ` take a label
- Labels: `name:` defines a label (target for jumps and calls), case-insensitive

## Instruction Set

| Byte | Mnemonic | Operand | Effect |
|------|----------|---------|--------|
| 0x01 | `PUSH n` | i64 | Push `n` |
| 0x02 | `POP` | - | Discard top |
| 0x03 | `DUP` | - | Duplicate top |
| 0x04 | `SWAP` | - | Swap top two |
| 0x10 | `ADD` | - | Pop b, pop a, push a+b |
| 0x11 | `SUB` | - | Pop b, pop a, push a-b |
| 0x12 | `MUL` | - | Pop b, pop a, push a\*b |
| 0x13 | `DIV` | - | Pop b, pop a, push a/b (trap if b=0) |
| 0x14 | `MOD` | - | Pop b, pop a, push a%b (trap if b=0) |
| 0x15 | `NEG` | - | Pop a, push -a |
| 0x40 | `LOAD s` | u8 slot | Push global slot `s` |
| 0x41 | `STORE s` | u8 slot | Pop into global slot `s` |
| 0x60 | `PRINT` | - | Pop and print with newline |
| 0x20 | `EQ` | - | Pop b, pop a, push 1 if a==b else 0 |
| 0x21 | `LT` | - | Pop b, pop a, push 1 if a&lt;b else 0 |
| 0x22 | `GT` | - | Pop b, pop a, push 1 if a&gt;b else 0 |
| 0x30 | `JMP label` | u32 | Unconditional jump to label |
| 0x31 | `JZ label` | u32 | Pop, jump to label if zero |
| 0x32 | `JNZ label` | u32 | Pop, jump to label if non-zero |
| 0x33 | `CALL label` | u32 | Push return address, jump to label |
| 0x34 | `RET` | - | Pop return address, jump back |
| 0xFF | `HALT` | - | Stop execution |

## Bytecode Format (.tbc)

All multi-byte values are little-endian.

```
[ MAGIC: 4 bytes ][ VERSION: 1 byte ][ LENGTH: 4 bytes ][ CODE: N bytes ]
   M V M \0            0x01                u32 LE            raw opcodes
```

Validation checks: magic bytes, version number, and length field must all match.

## Example Programs

Example `.tasm` files are in `examples/`. See the translation table below for each program's logic and expected output.

## Infix to Stack Translation

| Expression | Stack Code | Output |
|---|---|---|
| `(7 + 3) * (9 - 4) / 5` | `PUSH 7, PUSH 3, ADD, PUSH 9, PUSH 4, SUB, MUL, PUSH 5, DIV` | 10 |
| `3·11³ + 2·11² + 5·11 + 7` | `PUSH 11, STORE 0, PUSH 3, LOAD 0, MUL, PUSH 2, ADD, LOAD 0, MUL, PUSH 5, ADD, LOAD 0, MUL, PUSH 7, ADD` | 4297 |
| `100 × 9 / 5 + 32` | `PUSH 100, PUSH 9, MUL, PUSH 5, DIV, PUSH 32, ADD` | 212 |
| `12² + 35²` | `PUSH 12, STORE 0, PUSH 35, STORE 1, LOAD 0, LOAD 0, MUL, LOAD 1, LOAD 1, MUL, ADD` | 1369 |
| digits of 9274 | `PUSH 9274, STORE 0, LOAD 0, PUSH 1000, DIV, PRINT, ...` | 9 2 7 4 |
| `∑₁₀₀` | `PUSH 100, STORE 0, ..., LOOP: LOAD 0, JZ DONE, ... JMP LOOP, DONE: ...` | 5050 |
| 5! | `PUSH 5, STORE 0, PUSH 1, STORE 1, LOOP: ..., JMP LOOP, DONE: LOAD 1, PRINT` | 120 |
| gcd(56, 98) | `PUSH 56, STORE 0, PUSH 98, STORE 1, LOOP: ..., JZ DONE, ..., JMP LOOP, DONE: ...` | 14 |
| max(17, 42, 8) | `PUSH 17, STORE 0, PUSH 42, STORE 1, PUSH 8, STORE 2, ..., GT, JZ, ...` | 42 |
| F(20) | `PUSH 20, STORE 0, PUSH 0, STORE 1, PUSH 1, STORE 2, LOOP: ..., JMP LOOP, DONE: LOAD 1, PRINT` | 6765 |
| 2¹⁰ | `PUSH 2, STORE 0, PUSH 10, STORE 1, PUSH 1, STORE 2, LOOP: ..., JMP LOOP, DONE: LOAD 2, PRINT` | 1024 |
| is 97 prime? | `PUSH 97, STORE 0, PUSH 2, STORE 1, LOOP: ..., GT, JZ, MOD, JZ, ..., JMP LOOP` | 1 |
| 5! (recursive) | `PUSH 5, CALL fact, fact: DUP, PUSH 1, GT, JZ base, DUP, PUSH 1, SUB, CALL fact, MUL, RET, base: POP, PUSH 1, RET` | 120 |

## Trap Handling

The VM detects 5 classes of runtime errors and reports them with the instruction pointer:

| Trap | Example Trigger | Output |
|---|---|---|
| Stack underflow | `POP` on empty stack | `trap at ip=0x0000: stack underflow (POP on empty stack)` |
| Stack overflow | 1025 values pushed | `trap at ip=0x2400: stack overflow (PUSH on full stack)` |
| Division by zero | `DIV` with b=0 | `trap at ip=0x0012: division by zero` |
| Modulo by zero | `MOD` with b=0 | `trap at ip=0x0012: modulo by zero` |
| Missing HALT | program ends without HALT | `trap at ip=0x000A: program ended without HALT` |

Each trap is reported to stderr and the process exits with a non-zero status.

Trap test cases are in `traps/`:
- `traps/div_zero.tasm`
- `traps/mod_zero.tasm`
- `traps/stack_underflow.tasm`
- `traps/stack_overflow.tasm`
- `traps/missing_halt.tasm`

## Project Structure

```
virtualMachine/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI: asm / run / run --trace / run --step / dis
│   ├── isa.rs           # Op enum, encode, decode (sole source of byte values)
│   ├── bytecode.rs      # .tbc file format (magic, version, length)
│   ├── assembler.rs     # .tasm → bytecode
│   ├── vm.rs            # stack machine executor + trap handling
│   └── disassembler.rs  # bytecode → .tasm text
├── examples/
│   ├── arithmetic.tasm
│   ├── celsius.tasm
│   ├── digits.tasm
│   ├── fact_rec.tasm
│   ├── factorial.tasm
│   ├── fibonacci.tasm
│   ├── gcd.tasm
│   ├── horner.tasm
│   ├── max3.tasm
│   ├── power.tasm
│   ├── prime.tasm
│   ├── stackplay.tasm
│   └── sum.tasm
└── traps/
    ├── div_zero.tasm
    ├── missing_halt.tasm
    ├── mod_zero.tasm
    ├── stack_overflow.tasm
    └── stack_underflow.tasm
```

## Design Decisions

- **`isa.rs` is the single source of truth** for byte encodings. Assembler, disassembler, and VM all go through `Op::encode` / `Op::decode`. No magic byte values appear in more than one place.
- **256 global i64 slots**, zero-initialized, for persistent state between instructions.
- **Operand stack capped at 1024** values to prevent runaway memory usage.
- **Separate return stack** — `CALL` pushes the return address to a dedicated `ret_stack` (not the data stack), preventing data/address corruption in nested or recursive calls.
- **Two-pass assembler** — first pass collects label definitions with byte offsets, second pass emits code with resolved addresses.

## Build & Run

```sh
cargo build --release
cargo run -- asm examples/arithmetic.tasm -o examples/arithmetic.tbc
cargo run -- run examples/arithmetic.tbc
cargo run -- dis examples/arithmetic.tbc
```

Round-trip test: `asm → dis → asm` produces byte-identical output.
