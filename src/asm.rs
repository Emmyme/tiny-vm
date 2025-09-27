use crate::vm::Instruction;
use std::collections::HashMap;

pub fn assemble(source: &str) -> Result<Vec<Instruction>, String> {
    let mut program = Vec::new();
    let mut labels = HashMap::new();
    let mut unresolved = Vec::new();

    let lines: Vec<&str> = source
        .lines()
        .map(|l| {
            if let Some(comment_pos) = l.find(';') { l[..comment_pos].trim() } else { l.trim() }
        })
        .filter(|l| !l.is_empty())
        .collect();

    let mut instruction_count = 0;
    for line in &lines {
        if line.ends_with(":") {
            let label = line.trim_end_matches(":").to_string();
            labels.insert(label, instruction_count);
        } else {
            instruction_count += 1;
        }
    }

    for line in &lines {
        if line.ends_with(":") {
            continue;
        }

        let parts: Vec<&str> = line
            .split_whitespace()
            .map(|part| part.trim_end_matches(','))
            .filter(|s| !s.is_empty())
            .collect();

        if parts.is_empty() {
            return Err("Empty instruction".to_string());
        }

        let instr = match parts[0].to_uppercase().as_str() {
            "LOADIMM" => {
                if parts.len() != 3 {
                    return Err("LOADIMM requires 2 arguments".to_string());
                }
                Instruction::LoadImm(parse_reg(parts[1])?, parse_number(parts[2])?)
            }
            "MOVE" => {
                if parts.len() != 3 {
                    return Err("MOVE requires 2 arguments".to_string());
                }
                Instruction::Move(parse_reg(parts[1])?, parse_reg(parts[2])?)
            }
            "ADD" => {
                if parts.len() != 4 {
                    return Err("ADD requires 3 arguments".to_string());
                }
                Instruction::Add(parse_reg(parts[1])?, parse_reg(parts[2])?, parse_reg(parts[3])?)
            }
            "SUB" => {
                if parts.len() != 4 {
                    return Err("SUB requires 3 arguments".to_string());
                }
                Instruction::Sub(parse_reg(parts[1])?, parse_reg(parts[2])?, parse_reg(parts[3])?)
            }
            "MUL" => {
                if parts.len() != 4 {
                    return Err("MUL requires 3 arguments".to_string());
                }
                Instruction::Mul(parse_reg(parts[1])?, parse_reg(parts[2])?, parse_reg(parts[3])?)
            }
            "DIV" => {
                if parts.len() != 4 {
                    return Err("DIV requires R2".to_string());
                }
                Instruction::Div(parse_reg(parts[1])?, parse_reg(parts[2])?, parse_reg(parts[3])?)
            }
            "STORE" => {
                if parts.len() != 3 {
                    return Err("STORE requires 100".to_string());
                }
                Instruction::Store(parse_reg(parts[1])?, parse_address(parts[2])?)
            }
            "LOAD" => {
                if parts.len() != 3 {
                    return Err("LOAD requires 100".to_string());
                }
                Instruction::Load(parse_reg(parts[1])?, parse_address(parts[2])?)
            }
            "JUMP" => {
                if parts.len() != 2 {
                    return Err("JUMP requires 1 argument".to_string());
                }
                parse_jump_instruction(
                    parts[1],
                    &labels,
                    &mut unresolved,
                    program.len(),
                    Instruction::Jump
                )?
            }
            "JUMPIFZERO" | "JZ" => {
                if parts.len() != 3 {
                    return Err("JUMPIFZERO requires label".to_string());
                }
                let reg = parse_reg(parts[1])?;
                if let Some(addr) = parse_label_or_number(parts[2], &labels) {
                    Instruction::JumpIfZero(reg, addr)
                } else {
                    unresolved.push((
                        program.len(),
                        parts[2].to_string(),
                        InstructionType::JumpIfZero(reg),
                    ));
                    Instruction::Halt
                }
            }
            "JUMPIFNOTZERO" | "JNZ" => {
                if parts.len() != 3 {
                    return Err("JUMPIFNOTZERO requires label".to_string());
                }
                let reg = parse_reg(parts[1])?;
                if let Some(addr) = parse_label_or_number(parts[2], &labels) {
                    Instruction::JumpIfNotZero(reg, addr)
                } else {
                    unresolved.push((
                        program.len(),
                        parts[2].to_string(),
                        InstructionType::JumpIfNotZero(reg),
                    ));
                    Instruction::Halt
                }
            }
            "JUMPIFNEGATIVE" | "JN" => {
                if parts.len() != 2 {
                    return Err(
                        "JUMPIFNEGATIVE requires 1 argument".to_string()
                    );
                }
                if let Some(addr) = parse_label_or_number(parts[1], &labels) {
                    Instruction::JumpIfNegative(addr)
                } else {
                    unresolved.push((
                        program.len(),
                        parts[1].to_string(),
                        InstructionType::JumpIfNegative,
                    ));
                    Instruction::Halt
                }
            }
            "JUMPIFPOSITIVE" | "JP" => {
                if parts.len() != 2 {
                    return Err(
                        "JUMPIFPOSITIVE requires 1 argument".to_string()
                    );
                }
                if let Some(addr) = parse_label_or_number(parts[1], &labels) {
                    Instruction::JumpIfPositive(addr)
                } else {
                    unresolved.push((
                        program.len(),
                        parts[1].to_string(),
                        InstructionType::JumpIfPositive,
                    ));
                    Instruction::Halt
                }
            }
            "CMP" => {
                if parts.len() != 3 {
                    return Err("CMP requires R1".to_string());
                }
                Instruction::Cmp(parse_reg(parts[1])?, parse_reg(parts[2])?)
            }
            "EQUAL" => {
                if parts.len() != 4 {
                    return Err("EQUAL requires R2".to_string());
                }
                Instruction::Equal(parse_reg(parts[1])?, parse_reg(parts[2])?, parse_reg(parts[3])?)
            }
            "LESS" => {
                if parts.len() != 4 {
                    return Err("LESS requires R2".to_string());
                }
                Instruction::Less(parse_reg(parts[1])?, parse_reg(parts[2])?, parse_reg(parts[3])?)
            }
            "PUSH" => {
                if parts.len() != 2 {
                    return Err("PUSH requires 1 argument".to_string());
                }
                Instruction::Push(parse_reg(parts[1])?)
            }
            "POP" => {
                if parts.len() != 2 {
                    return Err("POP requires 1 argument".to_string());
                }
                Instruction::Pop(parse_reg(parts[1])?)
            }
            "CALL" => {
                if parts.len() != 2 {
                    return Err("CALL requires 1 argument".to_string());
                }
                if let Some(addr) = parse_label_or_number(parts[1], &labels) {
                    Instruction::Call(addr)
                } else {
                    unresolved.push((program.len(), parts[1].to_string(), InstructionType::Call));
                    Instruction::Halt
                }
            }
            "RET" => {
                if parts.len() != 1 {
                    return Err("RET takes no arguments".to_string());
                }
                Instruction::Ret
            }
            "PRINT" => {
                if parts.len() != 2 {
                    return Err("PRINT requires 1 argument".to_string());
                }
                Instruction::Print(parse_reg(parts[1])?)
            }
            "PRINTCHAR" => {
                if parts.len() != 2 {
                    return Err("PRINTCHAR requires 1 argument".to_string());
                }
                Instruction::PrintChar(parse_reg(parts[1])?)
            }
            "HALT" => {
                if parts.len() != 1 {
                    return Err("HALT takes no arguments".to_string());
                }
                Instruction::Halt
            }
            _ => {
                return Err(format!("Unknown instruction: {}", parts[0]));
            }
        };

        program.push(instr);
    }

    for (pos, label, instr_type) in unresolved {
        if let Some(&addr) = labels.get(&label) {
            program[pos] = match instr_type {
                InstructionType::Jump => Instruction::Jump(addr),
                InstructionType::JumpIfZero(reg) => Instruction::JumpIfZero(reg, addr),
                InstructionType::JumpIfNotZero(reg) => Instruction::JumpIfNotZero(reg, addr),
                InstructionType::JumpIfNegative => Instruction::JumpIfNegative(addr),
                InstructionType::JumpIfPositive => Instruction::JumpIfPositive(addr),
                InstructionType::Call => Instruction::Call(addr),
            };
        } else {
            return Err(format!("Undefined label: {}", label));
        }
    }

    Ok(program)
}

enum InstructionType {
    Jump,
    JumpIfZero(u8),
    JumpIfNotZero(u8),
    JumpIfNegative,
    JumpIfPositive,
    Call,
}

fn parse_reg(s: &str) -> Result<u8, String> {
    let s = s.to_uppercase();
    if !s.starts_with("R") {
        return Err(format!("Invalid register format: {}", s));
    }
    let n: u8 = s[1..].parse().map_err(|_| format!("Invalid register number: {}", s))?;
    if n >= 4 {
        return Err(format!("Invalid register: {}", s));
    }
    Ok(n)
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| format!("Invalid number: {}", s))
}

fn parse_address(s: &str) -> Result<usize, String> {
    s.parse::<usize>().map_err(|_| format!("Invalid address: {}", s))
}

fn parse_label_or_number(s: &str, labels: &HashMap<String, usize>) -> Option<usize> {
    if let Ok(addr) = s.parse::<usize>() {
        return Some(addr);
    }
    labels.get(s).copied()
}

fn parse_jump_instruction(
    target: &str,
    labels: &HashMap<String, usize>,
    unresolved: &mut Vec<(usize, String, InstructionType)>,
    position: usize,
    constructor: fn(usize) -> Instruction
) -> Result<Instruction, String> {
    if let Some(addr) = parse_label_or_number(target, labels) {
        Ok(constructor(addr))
    } else {
        unresolved.push((position, target.to_string(), InstructionType::Jump));
        Ok(Instruction::Halt)
    }
}
