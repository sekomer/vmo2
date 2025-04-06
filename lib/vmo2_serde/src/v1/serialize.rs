use crate::metadata::MAGIC;
use crate::traits::Serializable;
use crate::v1::opcode::*;
use vmo2_types::ast;
use vmo2_types::opcode::Opcode;
use vmo2_types::value::Value;

pub struct Serializer {
    magic_number: u32,
    version: u8,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            magic_number: MAGIC,
            version: 1,
        }
    }
}

impl Serializable for Serializer {
    type Output = Vec<u8>;

    fn serialize(&self, ast: &ast::Ast) -> Self::Output {
        let mut data = Vec::new();

        data.extend(self.magic_number.to_le_bytes());
        data.push(self.version);

        for opcode in ast.opcodes.iter() {
            match opcode {
                Opcode::Literal(value) => {
                    data.push(get_opcode_byte(opcode));
                    match value {
                        Value::UInt(v) => {
                            data.push(get_literal_opcode_byte(value));
                            data.extend(v.to_le_bytes());
                        }
                        Value::Bool(v) => {
                            data.push(get_literal_opcode_byte(value));
                            data.push(if *v { 1 } else { 0 });
                        }
                        Value::String(v) => {
                            data.push(get_literal_opcode_byte(value));
                            data.extend((v.len() as u16).to_le_bytes());
                            data.extend(v.as_bytes());
                        }
                    }
                }
                Opcode::Arithmetic(arith) => {
                    data.push(get_opcode_byte(opcode));
                    data.push(get_arithmetic_opcode_byte(arith));
                }
                Opcode::Logic(logic) => {
                    data.push(get_opcode_byte(opcode));
                    data.push(get_logic_opcode_byte(logic));
                }
                Opcode::Comparison(comparison) => {
                    data.push(get_opcode_byte(opcode));
                    data.push(get_comparison_opcode_byte(comparison));
                }
                Opcode::Memory(memory) => {
                    data.push(get_opcode_byte(opcode));
                    data.push(get_memory_opcode_byte(memory));
                }
                Opcode::IO(io) => {
                    data.push(get_opcode_byte(opcode));
                    data.push(get_io_opcode_byte(io));
                }
                Opcode::Halt => {
                    data.push(get_opcode_byte(opcode));
                }
            }
        }

        data
    }
}
