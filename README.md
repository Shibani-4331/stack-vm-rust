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
- Operands: `PUSH` takes an `i64`, `LOAD`/`STORE` take a `u8` slot number

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
| 0xFF | `HALT` | - | Stop execution |

## Bytecode Format (.tbc)

All multi-byte values are little-endian.

```
[ MAGIC: 4 bytes ][ VERSION: 1 byte ][ LENGTH: 4 bytes ][ CODE: N bytes ]
   M V M \0            0x01                u32 LE            raw opcodes
```

Validation checks: magic bytes, version number, and length field must all match.

## Example Programs

### Arithmetic — `(7 + 3) * (9 - 4) / 5`

```
PUSH 7          ; [7]
PUSH 3          ; [7, 3]
ADD             ; [10]
PUSH 9          ; [10, 9]
PUSH 4          ; [10, 9, 4]
SUB             ; [10, 5]
MUL             ; [50]
PUSH 5          ; [50, 5]
DIV             ; [10]
PRINT           ; []
HALT
```

Output: `10`

### Horner's Method — `3x³ + 2x² + 5x + 7` for x=11

```
PUSH 11
STORE 0         ; x = 11 in slot 0
PUSH 3          ; start with coefficient 3
LOAD 0          ; multiply by x
MUL
PUSH 2          ; add next coefficient
ADD
LOAD 0          ; multiply by x again
MUL
PUSH 5
ADD
LOAD 0
MUL
PUSH 7          ; add final coefficient
ADD
PRINT
HALT
```

Output: `4297`

### Celsius to Fahrenheit — `100°C × 9/5 + 32`

```
PUSH 100
PUSH 9
MUL
PUSH 5
DIV
PUSH 32
ADD
PRINT
HALT
```

Output: `212`

### Stack Play — `a² + b²` for a=12, b=35

```
PUSH 12
STORE 0         ; a = 12
PUSH 35
STORE 1         ; b = 35
LOAD 0
LOAD 0
MUL             ; a²
LOAD 1
LOAD 1
MUL             ; b²
ADD             ; a² + b²
PRINT
HALT
```

Output: `1369`

### Digits — Print digits of 9274 on separate lines

```
PUSH 9274
STORE 0
LOAD 0
PUSH 1000
DIV             ; 9
PRINT
LOAD 0
PUSH 100
DIV
PUSH 10
MOD             ; 2
PRINT
LOAD 0
PUSH 10
DIV
PUSH 10
MOD             ; 7
PRINT
LOAD 0
PUSH 10
MOD             ; 4
PRINT
HALT
```

Output:
```
9
2
7
4
```

## Infix to Stack Translation

| Expression | Stack Code | Output |
|---|---|---|
| `(7 + 3) * (9 - 4) / 5` | `PUSH 7, PUSH 3, ADD, PUSH 9, PUSH 4, SUB, MUL, PUSH 5, DIV` | 10 |
| `3·11³ + 2·11² + 5·11 + 7` | `PUSH 11, STORE 0, PUSH 3, LOAD 0, MUL, PUSH 2, ADD, LOAD 0, MUL, PUSH 5, ADD, LOAD 0, MUL, PUSH 7, ADD` | 4297 |
| `100 × 9 / 5 + 32` | `PUSH 100, PUSH 9, MUL, PUSH 5, DIV, PUSH 32, ADD` | 212 |
| `12² + 35²` | `PUSH 12, STORE 0, PUSH 35, STORE 1, LOAD 0, LOAD 0, MUL, LOAD 1, LOAD 1, MUL, ADD` | 1369 |
| digits of 9274 | `PUSH 9274, STORE 0, LOAD 0, PUSH 1000, DIV, PRINT, ...` | 9 2 7 4 |

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
│   ├── main.rs          # CLI: asm / run / run --trace / dis
│   ├── isa.rs           # Op enum, encode, decode (sole source of byte values)
│   ├── bytecode.rs      # .tbc file format (magic, version, length)
│   ├── assembler.rs     # .tasm → bytecode
│   ├── vm.rs            # stack machine executor + trap handling
│   └── disassembler.rs  # bytecode → .tasm text
├── examples/
│   ├── arithmetic.tasm
│   ├── horner.tasm
│   ├── celsius.tasm
│   ├── stackplay.tasm
│   └── digits.tasm
└── traps/
    ├── div_zero.tasm
    ├── mod_zero.tasm
    ├── stack_underflow.tasm
    ├── stack_overflow.tasm
    └── missing_halt.tasm
```

## Design Decisions

- **`isa.rs` is the single source of truth** for byte encodings. Assembler, disassembler, and VM all go through `Op::encode` / `Op::decode`. No magic byte values appear in more than one place.
- **256 global i64 slots**, zero-initialized, for persistent state between instructions.
- **Operand stack capped at 1024** values to prevent runaway memory usage.
- **Single-pass assembler** — no labels, no jumps, straight-line code only.

## Build & Run

```sh
cargo build --release
cargo run -- asm examples/arithmetic.tasm -o examples/arithmetic.tbc
cargo run -- run examples/arithmetic.tbc
cargo run -- dis examples/arithmetic.tbc
```

Round-trip test: `asm → dis → asm` produces byte-identical output.
