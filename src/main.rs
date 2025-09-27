mod vm;
mod asm;

use vm::VM;
use std::fs;

fn main() {
    println!("=== Tiny VM Assembly Programming Examples ===\n");

    run_example("basic_math.asm");
    run_example("fibonacci.asm");
    run_example("factorial.asm");
    run_example("string_output.asm");
    run_example("bubble_sort.asm");
    run_example("game_guess.asm");
}

fn run_example(filename: &str) {
    println!("Running example: {}", filename);
    println!("==================================================");

    let filepath = format!("examples/{}", filename);

    match fs::read_to_string(&filepath) {
        Ok(source_code) => {
            println!("Assembly code:");
            for (i, line) in source_code.lines().enumerate() {
                if !line.trim().is_empty() {
                    println!("{:3}: {}", i + 1, line);
                }
            }
            println!();

            match asm::assemble(&source_code) {
                Ok(program) => {
                    let mut vm = VM::new();
                    vm.load_program(program);

                    println!("Output:");
                    match vm.run() {
                        Ok(_) => println!("✓ Program completed successfully!"),
                        Err(e) => println!("✗ Runtime Error: {}", e),
                    }
                }
                Err(e) => println!("✗ Assembly Error: {}", e),
            }
        }
        Err(_) => {
            println!("Example file '{}' not found.", filename);
        }
    }

    println!("--------------------------------------------------\n");
}
