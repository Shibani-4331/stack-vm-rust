# Stack VM in Rust

## Overview

This project implements a stack-based virtual machine (VM) in Rust using only the standard library. Programs are written in a custom assembly language (`.tasm`), assembled into bytecode (`.tbc`), and executed by the VM.

Pipeline:

program.tasm → Assembler → program.tbc → VM → Output

The VM uses:

* 64-bit signed integers (`i64`)
* Stack size limit: 1024 values
* 256 global variables
* Little-endian bytecode encoding

---

## Features

* Assembler (`asm`)
* Virtual Machine (`run`)
* Trace mode (`trace`)
* Disassembler (`dis`)
* Bytecode validation
* Error trapping
* Round-trip verification (`asm → dis → asm`)

---

## Commands

Assemble:

cargo run -- asm program.tasm -o program.tbc

Run:

cargo run -- run program.tbc

Trace execution:

cargo run -- trace program.tbc

Disassemble:

cargo run -- dis program.tbc

---

## Instruction Set

| Opcode | Instruction | Description       |
| ------ | ----------- | ----------------- |
| 0x01   | PUSH n      | Push value        |
| 0x02   | POP         | Remove top        |
| 0x03   | DUP         | Duplicate top     |
| 0x04   | SWAP        | Swap top two      |
| 0x10   | ADD         | Addition          |
| 0x11   | SUB         | Subtraction       |
| 0x12   | MUL         | Multiplication    |
| 0x13   | DIV         | Division          |
| 0x14   | MOD         | Modulo            |
| 0x15   | NEG         | Negation          |
| 0x40   | LOAD s      | Load global slot  |
| 0x41   | STORE s     | Store global slot |
| 0x60   | PRINT       | Print value       |
| 0xFF   | HALT        | Stop execution    |

---

## Bytecode Format

Header:

* Magic: MVM\0
* Version: 0x01
* Code length: u32 (little-endian)

Layout:

[MAGIC][VERSION][LENGTH][CODE]

---

## Acceptance Tests

* arithmetic.tasm → prints 10
* horner.tasm → prints 4297
* celsius.tasm → prints 212
* stackplay.tasm → prints 1369
* digits.tasm → prints digits of 9274

---

## Trap Handling

The VM detects:

* Stack underflow
* Stack overflow
* Division by zero
* Modulo by zero
* Invalid opcode
* Truncated instruction
* Missing HALT

Example:

trap at ip=0x0012: division by zero

---

## Infix to Stack Translation

| Expression  | Stack Code                                    |
| ----------- | --------------------------------------------- |
| 7 + 3       | PUSH 7, PUSH 3, ADD                           |
| (7 + 3) * 5 | PUSH 7, PUSH 3, ADD, PUSH 5, MUL              |
| a² + b²     | LOAD a, LOAD a, MUL, LOAD b, LOAD b, MUL, ADD |
| -(x)        | LOAD x, NEG                                   |

---

## Project Structure

src/
examples/
traps/
README.md

---

## Technologies

* Rust
* Standard Library Only
