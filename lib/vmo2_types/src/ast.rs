use crate::opcode::Opcode;
use quickcheck::{Arbitrary, Gen};
use rand::{Rng, thread_rng};

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Arbitrary for Ast {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut rng = thread_rng();
        let mut ast = Ast::new();

        for _ in 0..rng.gen_range(0..256) {
            ast.add_opcode(Opcode::arbitrary(g));
        }

        ast.opcodes.push(Opcode::Halt);

        ast
    }
}
