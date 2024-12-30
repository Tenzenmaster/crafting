use crate::*;

#[derive(Debug)]
pub enum Opcode {
    Return,
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Default)]
pub struct Chunk {
    pub code: Vec<Opcode>,
    pub constants: Vec<Value>,
    pub lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_code(&mut self, code: Opcode, line: u32) {
        self.code.push(code);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, constant: Value, line: u32) {
        let index = self.constants.len();
        self.write_code(Opcode::Constant(index), line);
        self.constants.push(constant);
    }
}
