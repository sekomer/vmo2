use crate::v1::constants::OPCODE;
use vmo2_types::opcode::{
    ArithmeticOpcode, ComparisonOpcode, IOOpcode, LogicOpcode, MemoryOpcode, Opcode,
};
use vmo2_types::value::Value;

pub fn get_opcode_byte(opcode: &Opcode) -> u8 {
    match opcode {
        Opcode::Halt => OPCODE::HALT,
        Opcode::Literal(_) => OPCODE::LITERAL,
        Opcode::Arithmetic(_) => OPCODE::ARITHMETIC,
        Opcode::Logic(_) => OPCODE::LOGIC,
        Opcode::Comparison(_) => OPCODE::COMPARISON,
        Opcode::Memory(_) => OPCODE::MEMORY,
        Opcode::IO(_) => OPCODE::IO,
    }
}

pub fn get_literal_opcode_byte(value: &Value) -> u8 {
    match value {
        Value::UInt(_) => OPCODE::LITERAL_UINT,
        Value::Bool(_) => OPCODE::LITERAL_BOOL,
        Value::String(_) => OPCODE::LITERAL_STRING,
    }
}

pub fn get_arithmetic_opcode_byte(opcode: &ArithmeticOpcode) -> u8 {
    match opcode {
        ArithmeticOpcode::Add => OPCODE::ARITHMETIC_ADD,
        ArithmeticOpcode::Sub => OPCODE::ARITHMETIC_SUB,
        ArithmeticOpcode::Mul => OPCODE::ARITHMETIC_MUL,
        ArithmeticOpcode::Div => OPCODE::ARITHMETIC_DIV,
    }
}

pub fn get_logic_opcode_byte(opcode: &LogicOpcode) -> u8 {
    match opcode {
        LogicOpcode::And => OPCODE::LOGIC_AND,
        LogicOpcode::Or => OPCODE::LOGIC_OR,
        LogicOpcode::Xor => OPCODE::LOGIC_XOR,
        LogicOpcode::Not => OPCODE::LOGIC_NOT,
    }
}

pub fn get_comparison_opcode_byte(opcode: &ComparisonOpcode) -> u8 {
    match opcode {
        ComparisonOpcode::Eq => OPCODE::COMPARISON_EQ,
        ComparisonOpcode::Ne => OPCODE::COMPARISON_NE,
        ComparisonOpcode::Lt => OPCODE::COMPARISON_LT,
        ComparisonOpcode::Le => OPCODE::COMPARISON_LE,
        ComparisonOpcode::Gt => OPCODE::COMPARISON_GT,
        ComparisonOpcode::Ge => OPCODE::COMPARISON_GE,
    }
}

pub fn get_memory_opcode_byte(opcode: &MemoryOpcode) -> u8 {
    match opcode {
        MemoryOpcode::Load => OPCODE::MEMORY_LOAD,
        MemoryOpcode::Store => OPCODE::MEMORY_STORE,
    }
}

pub fn get_io_opcode_byte(opcode: &IOOpcode) -> u8 {
    match opcode {
        IOOpcode::Print => OPCODE::IO_PRINT,
        IOOpcode::Scan => OPCODE::IO_SCAN,
    }
}
