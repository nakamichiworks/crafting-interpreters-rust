use crate::common::OpCode;

pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new() }
    }

    pub fn init(&mut self) {
        self.code = Vec::new();
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        for _ in 0..self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let instruction = &self.code[offset];
        match instruction {
            OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset),
            _ => {
                println!("Unknown opcode {:?}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }
}
