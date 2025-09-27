# Tiny VM

A simple virtual machine implementation in Rust with assembly language support. It features 4 registers, 256 bytes of memory, and a built-in assembler. The project includes some example programs.

## Features

- 4 registers (R0-R3)
- 256 bytes of memory
- 23 instruction types
- Built-in assembler
- Stack operations
- Conditional jumps

## Usage

Build and run:
```bash
cargo run
```

## Assembly Example

```assembly
LOADIMM R0, 5         ; Load 5 into R0
LOADIMM R1, 10        ; Load 10 into R1
ADD R0, R1, R2        ; R2 = R0 + R1 = 15
PRINT R2              ; Print 15
HALT
```

## Examples

The `examples/` directory contains:
- basic_math.asm - Arithmetic operations
- fibonacci.asm - Fibonacci sequence
- factorial.asm - Factorial calculation
- string_output.asm - Text output
- bubble_sort.asm - Array sorting
- game_guess.asm - Number guessing game