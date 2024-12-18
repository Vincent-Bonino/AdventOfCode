#[derive(Clone, Debug)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    pub fn from_opcode(op_code: u8) -> Self {
        match op_code {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!("Invalid opcode '{op_code}"),
        }
    }
}

#[derive(Clone, Default)]
pub struct Computer {
    // Registers
    pub a: u32,
    pub b: u32,
    pub c: u32,

    // Stack
    jump_flag: bool,
    pub stack: Vec<u8>,
    stack_pointer: usize,

    stdout: Vec<u8>,
}

impl Computer {
    pub fn new(a: u32, b: u32, c: u32, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            jump_flag: true,
            stack: program,
            stack_pointer: 0,
            stdout: Vec::new(),
        }
    }

    pub fn with_a(&self, new_a: u32) -> Self {
        Self {
            a: new_a,
            b: self.b,
            c: self.c,
            jump_flag: true,
            stack: self.stack.clone(),
            stack_pointer: 0,
            stdout: Vec::new(),
        }
    }

    // Run methods

    pub fn run(&mut self) -> Vec<u8> {
        while self.stack_pointer < self.stack.len() {
            // Reset jump flag
            self.jump_flag = true;

            // Run next instruction
            let instruction: Instruction = Instruction::from_opcode(self.stack[self.stack_pointer]);
            let operand: u8 = self.stack[self.stack_pointer + 1];
            self.run_instruction(&instruction, operand);

            // Continue to next instruction
            if self.jump_flag {
                self.stack_pointer += 2
            }
        }

        self.stdout.clone()
    }

    pub fn format_stdout(stdout: &Vec<u8>) -> String {
        let str_stdout: Vec<String> = stdout.iter().map(|val| val.to_string()).collect();
        str_stdout.join(",")
    }

    // Decompiler methods

    #[allow(dead_code)]
    pub fn decompile(&self) {
        println!(">> {:?}", self.stack);
        println!("START");
        for pointer in 0..(self.stack.len() / 2) {
            let instruction: Instruction = Instruction::from_opcode(self.stack[2 * pointer]);
            let operand: u8 = self.stack[2 * pointer + 1];

            match instruction {
                Instruction::Adv => println!("A = A >> {}", Computer::decompile_combo(&operand)),
                Instruction::Bxl => println!("B = B ^ {operand}"),
                Instruction::Bst => println!("B = {} & 7", Computer::decompile_combo(&operand)),
                Instruction::Jnz => println!("<jump to {operand}>",),
                Instruction::Bxc => println!("B = B ^ C"),
                Instruction::Out => println!("print({})", Computer::decompile_combo(&operand)),
                Instruction::Bdv => println!("B = A >> {}", Computer::decompile_combo(&operand)),
                Instruction::Cdv => println!("C = A >> {}", Computer::decompile_combo(&operand)),
            };
        }
        println!("END");
    }

    #[allow(dead_code)]
    fn decompile_combo(combo: &u8) -> String {
        match combo {
            0 | 1 | 2 | 3 => combo.to_string(),
            4 => String::from("A"),
            5 => String::from("B"),
            6 => String::from("C"),
            7 => unimplemented!("Combo code 7 is reserved"),
            _ => unreachable!("Invalid combo value '{combo}'"),
        }
    }

    // Instructions

    fn run_instruction(&mut self, instruction: &Instruction, operand: u8) {
        match instruction {
            Instruction::Adv => self.run_adv(operand),
            Instruction::Bxl => self.run_bxl(operand),
            Instruction::Bst => self.run_bst(operand),
            Instruction::Jnz => self.run_jnz(operand),
            Instruction::Bxc => self.run_bxc(operand),
            Instruction::Out => self.run_out(operand),
            Instruction::Bdv => self.run_bdv(operand),
            Instruction::Cdv => self.run_cdv(operand),
        }
    }

    fn resolve_combo(&self, combo: u8) -> u32 {
        match combo {
            0 | 1 | 2 | 3 => combo as u32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unimplemented!("Combo code 7 is reserved"),
            _ => unreachable!("Invalid combo value '{combo}'"),
        }
    }

    /// Division (opcode=0)
    #[inline]
    fn run_adv(&mut self, operand: u8) {
        self.a = self.a >> self.resolve_combo(operand)
    }

    /// Bitwise XOR (opcode=1)
    #[inline]
    fn run_bxl(&mut self, operand: u8) {
        self.b = self.b ^ (operand as u32)
    }

    /// Modulo (opcode=2)
    #[inline]
    fn run_bst(&mut self, operand: u8) {
        self.b = self.resolve_combo(operand) & 7
    }

    /// Jump (opcode=3)
    #[inline]
    fn run_jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.stack_pointer = operand as usize;

            // Do not move stack pointer after jump
            self.jump_flag = false;
        }
    }

    /// Bitwise XOR (opcode=4)
    #[inline]
    fn run_bxc(&mut self, _operand: u8) {
        self.b = self.b ^ self.c
    }

    /// Write to stdout (opcode=5)
    #[inline]
    fn run_out(&mut self, operand: u8) {
        self.stdout.push((self.resolve_combo(operand) & 7) as u8);
    }

    /// Division (opcode=6)
    #[inline]
    fn run_bdv(&mut self, operand: u8) {
        self.b = self.a >> self.resolve_combo(operand)
    }

    /// Division (opcode=7)
    #[inline]
    fn run_cdv(&mut self, operand: u8) {
        self.c = self.a >> self.resolve_combo(operand)
    }
}
