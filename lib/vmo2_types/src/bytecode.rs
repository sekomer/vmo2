use crate::opcode::Opcode;
use quickcheck::{Arbitrary, Gen};
use rand::{Rng, thread_rng};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ByteCode {
    pub opcodes: Vec<Opcode>,
}

impl ByteCode {
    pub fn new() -> Self {
        Self { opcodes: vec![] }
    }

    pub fn add_opcode(&mut self, opcode: Opcode) {
        self.opcodes.push(opcode);
    }
}

impl From<Vec<Opcode>> for ByteCode {
    fn from(opcodes: Vec<Opcode>) -> Self {
        Self { opcodes }
    }
}

impl Arbitrary for ByteCode {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut rng = thread_rng();
        let mut bytecode = ByteCode::new();

        for _ in 0..rng.gen_range(0..256) {
            bytecode.add_opcode(Opcode::arbitrary(g));
        }

        bytecode.opcodes.push(Opcode::Halt);

        bytecode
    }
}
