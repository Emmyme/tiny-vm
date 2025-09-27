pub struct VM {
    pub registers: [i32; 4],
    pub memory: [u8; 256],
    pub pc: usize,
    pub sp: usize,
    pub program: Vec<Instruction>,
    pub halted: bool,
    pub zero_flag: bool,
    pub negative_flag: bool,
    pub carry_flag: bool,
    pub overflow_flag: bool,
}

#[derive(Clone, Debug)]
pub enum Instruction {
    LoadImm(u8, i32),
    Move(u8, u8),
    Load(u8, usize),
    Store(u8, usize),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Div(u8, u8, u8),
    Cmp(u8, u8),
    Equal(u8, u8, u8),
    Less(u8, u8, u8),
    Jump(usize),
    JumpIfZero(u8, usize),
    JumpIfNotZero(u8, usize),
    JumpIfNegative(usize),
    JumpIfPositive(usize),
    Call(usize),
    Ret,
    Push(u8),
    Pop(u8),
    Print(u8),
    PrintChar(u8),
    Halt,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 4],
            memory: [0; 256],
            pc: 0,
            sp: 256,
            program: Vec::new(),
            halted: false,
            zero_flag: false,
            negative_flag: false,
            carry_flag: false,
            overflow_flag: false,
        }
    }

    fn update_flags(&mut self, result: i32) {
        self.zero_flag = result == 0;
        self.negative_flag = result < 0;
        self.carry_flag = false;
        self.overflow_flag = false;
    }

    fn stack_push(&mut self, value: i32) -> Result<(), String> {
        if self.sp < 4 {
            return Err("Stack overflow".to_string());
        }
        self.sp -= 4;
        let bytes = value.to_le_bytes();
        for i in 0..4 {
            self.memory[self.sp + i] = bytes[i];
        }
        Ok(())
    }

    fn stack_pop(&mut self) -> Result<i32, String> {
        if self.sp >= 252 {
            return Err("Stack underflow".to_string());
        }

        let bytes = [
            self.memory[self.sp],
            self.memory[self.sp + 1],
            self.memory[self.sp + 2],
            self.memory[self.sp + 3],
        ];

        self.sp += 4;

        Ok(i32::from_le_bytes(bytes))
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.pc = 0;
        self.halted = false;
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.halted {
            return Err("VM is halted".to_string());
        }
        if self.pc >= self.program.len() {
            self.halted = true;
            return Err("Program counter out of bounds".to_string());
        }

        let instruction = self.program[self.pc].clone();
        self.execute_instruction(instruction)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        match instruction {
            Instruction::LoadImm(reg, value) => {
                self.validate_register(reg)?;
                self.registers[reg as usize] = value;
                self.pc += 1;
            }
            Instruction::Move(from_reg, to_reg) => {
                self.validate_register(from_reg)?;
                self.validate_register(to_reg)?;
                self.registers[to_reg as usize] = self.registers[from_reg as usize];
                self.pc += 1;
            }
            Instruction::Add(reg1, reg2, result_reg) => {
                self.validate_register(reg1)?;
                self.validate_register(reg2)?;
                self.validate_register(result_reg)?;

                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                self.registers[result_reg as usize] = val1 + val2;
                self.pc += 1;
            }
            Instruction::Sub(reg1, reg2, result_reg) => {
                self.validate_register(reg1)?;
                self.validate_register(reg2)?;
                self.validate_register(result_reg)?;

                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                self.registers[result_reg as usize] = val1 - val2;
                self.pc += 1;
            }
            Instruction::Store(reg, addr) => {
                self.validate_register(reg)?;
                self.validate_memory_address(addr)?;

                let bytes = self.registers[reg as usize].to_le_bytes();
                for i in 0..4 {
                    self.memory[addr + i] = bytes[i];
                }
                self.pc += 1;
            }
            Instruction::Load(reg, addr) => {
                self.validate_register(reg)?;
                self.validate_memory_address(addr)?;

                let bytes = [
                    self.memory[addr],
                    self.memory[addr + 1],
                    self.memory[addr + 2],
                    self.memory[addr + 3],
                ];
                self.registers[reg as usize] = i32::from_le_bytes(bytes);
                self.pc += 1;
            }
            Instruction::Jump(addr) => {
                if addr >= self.program.len() {
                    return Err("Jump address out of bounds".to_string());
                }
                self.pc = addr;
            }
            Instruction::JumpIfZero(reg, addr) => {
                self.validate_register(reg)?;
                if addr >= self.program.len() {
                    return Err("Jump address out of bounds".to_string());
                }
                if self.registers[reg as usize] == 0 {
                    self.pc = addr;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Mul(reg1, reg2, result_reg) => {
                self.validate_register(reg1)?;
                self.validate_register(reg2)?;
                self.validate_register(result_reg)?;

                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                let result = val1 * val2;
                self.registers[result_reg as usize] = result;
                self.update_flags(result);
                self.pc += 1;
            }
            Instruction::Div(reg1, reg2, result_reg) => {
                self.validate_register(reg1)?;
                self.validate_register(reg2)?;
                self.validate_register(result_reg)?;

                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                if val2 == 0 {
                    return Err("Division by zero".to_string());
                }
                let result = val1 / val2;
                self.registers[result_reg as usize] = result;
                self.update_flags(result);
                self.pc += 1;
            }
            Instruction::Cmp(reg1, reg2) => {
                self.validate_register(reg1)?;
                self.validate_register(reg2)?;

                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                let result = val1 - val2;
                self.update_flags(result);
                self.pc += 1;
            }
            Instruction::Equal(reg1, reg2, result_reg) => {
                self.validate_register(reg1)?;
                self.validate_register(reg2)?;
                self.validate_register(result_reg)?;

                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                self.registers[result_reg as usize] = if val1 == val2 { 1 } else { 0 };
                self.pc += 1;
            }
            Instruction::Less(reg1, reg2, result_reg) => {
                self.validate_register(reg1)?;
                self.validate_register(reg2)?;
                self.validate_register(result_reg)?;

                let val1 = self.registers[reg1 as usize];
                let val2 = self.registers[reg2 as usize];
                self.registers[result_reg as usize] = if val1 < val2 { 1 } else { 0 };
                self.pc += 1;
            }
            Instruction::JumpIfNotZero(reg, addr) => {
                self.validate_register(reg)?;
                if addr >= self.program.len() {
                    return Err("Jump address out of bounds".to_string());
                }
                if self.registers[reg as usize] != 0 {
                    self.pc = addr;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::JumpIfNegative(addr) => {
                if addr >= self.program.len() {
                    return Err("Jump address out of bounds".to_string());
                }
                if self.negative_flag {
                    self.pc = addr;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::JumpIfPositive(addr) => {
                if addr >= self.program.len() {
                    return Err("Jump address out of bounds".to_string());
                }
                if !self.negative_flag && !self.zero_flag {
                    self.pc = addr;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Push(reg) => {
                self.validate_register(reg)?;
                let value = self.registers[reg as usize];
                self.stack_push(value)?;
                self.pc += 1;
            }
            Instruction::Pop(reg) => {
                self.validate_register(reg)?;
                let value = self.stack_pop()?;
                self.registers[reg as usize] = value;
                self.pc += 1;
            }
            Instruction::Call(addr) => {
                if addr >= self.program.len() {
                    return Err("Call address out of bounds".to_string());
                }
                self.stack_push((self.pc + 1) as i32)?;
                self.pc = addr;
            }
            Instruction::Ret => {
                let return_addr = self.stack_pop()? as usize;
                if return_addr >= self.program.len() {
                    return Err("Return address out of bounds".to_string());
                }
                self.pc = return_addr;
            }
            Instruction::Print(reg) => {
                self.validate_register(reg)?;
                println!("{}", self.registers[reg as usize]);
                self.pc += 1;
            }
            Instruction::PrintChar(reg) => {
                self.validate_register(reg)?;
                let value = self.registers[reg as usize];
                if value >= 0 && value <= 255 {
                    print!("{}", value as u8 as char);
                } else {
                    return Err("Invalid character value".to_string());
                }
                self.pc += 1;
            }
            Instruction::Halt => {
                self.halted = true;
            }
        }
        Ok(())
    }
    fn validate_register(&self, reg: u8) -> Result<(), String> {
        if reg >= 4 { Err(format!("Invalid register: {}", reg)) } else { Ok(()) }
    }
    fn validate_memory_address(&self, addr: usize) -> Result<(), String> {
        if addr + 3 >= self.memory.len() {
            Err(format!("Invalid memory address: {}", addr))
        } else {
            Ok(())
        }
    }
}
