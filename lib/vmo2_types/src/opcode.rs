use crate::value::Value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Opcode {
    Halt,
    Literal(Value),
    Arithmetic(ArithmeticOpcode),
    Logic(LogicOpcode),
    Comparison(ComparisonOpcode),
    Memory(MemoryOpcode),
    IO(IOOpcode),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArithmeticOpcode {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LogicOpcode {
    And,
    Xor,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComparisonOpcode {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MemoryOpcode {
    Load,
    Store,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IOOpcode {
    Print,
    Scan,
}
