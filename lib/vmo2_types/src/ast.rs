use crate::opcode::Opcode;

#[derive(Debug, PartialEq, Eq)]
pub struct Ast {
    pub opcodes: Vec<Opcode>,
}

impl Ast {
    pub fn new() -> Self {
        Self { opcodes: vec![] }
    }

    pub fn add_opcode(&mut self, opcode: Opcode) {
        self.opcodes.push(opcode);
    }
}

impl From<Vec<Opcode>> for Ast {
    fn from(opcodes: Vec<Opcode>) -> Self {
        Self { opcodes }
    }
}
