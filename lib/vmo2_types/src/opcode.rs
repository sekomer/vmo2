use quickcheck::{Arbitrary, Gen};
use rand::{seq::SliceRandom, thread_rng};

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

impl Arbitrary for Opcode {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut rng = thread_rng();
        let value = [1, 2, 3, 4, 5, 6, 7].choose(&mut rng).unwrap();

        match value {
            1 => Opcode::Halt,
            2 => Opcode::Literal(Value::arbitrary(g)),
            3 => Opcode::Arithmetic(ArithmeticOpcode::arbitrary(g)),
            4 => Opcode::Logic(LogicOpcode::arbitrary(g)),
            5 => Opcode::Comparison(ComparisonOpcode::arbitrary(g)),
            6 => Opcode::Memory(MemoryOpcode::arbitrary(g)),
            7 => Opcode::IO(IOOpcode::arbitrary(g)),
            _ => unreachable!(),
        }
    }
}

impl Arbitrary for ArithmeticOpcode {
    fn arbitrary(g: &mut Gen) -> Self {
        g.choose(&[
            ArithmeticOpcode::Add,
            ArithmeticOpcode::Sub,
            ArithmeticOpcode::Mul,
            ArithmeticOpcode::Div,
        ])
        .unwrap()
        .clone()
    }
}

impl Arbitrary for LogicOpcode {
    fn arbitrary(g: &mut Gen) -> Self {
        g.choose(&[
            LogicOpcode::And,
            LogicOpcode::Xor,
            LogicOpcode::Or,
            LogicOpcode::Not,
        ])
        .unwrap()
        .clone()
    }
}

impl Arbitrary for ComparisonOpcode {
    fn arbitrary(g: &mut Gen) -> Self {
        g.choose(&[
            ComparisonOpcode::Eq,
            ComparisonOpcode::Ne,
            ComparisonOpcode::Lt,
            ComparisonOpcode::Le,
            ComparisonOpcode::Gt,
            ComparisonOpcode::Ge,
        ])
        .unwrap()
        .clone()
    }
}

impl Arbitrary for MemoryOpcode {
    fn arbitrary(g: &mut Gen) -> Self {
        g.choose(&[MemoryOpcode::Load, MemoryOpcode::Store])
            .unwrap()
            .clone()
    }
}

impl Arbitrary for IOOpcode {
    fn arbitrary(g: &mut Gen) -> Self {
        g.choose(&[IOOpcode::Print, IOOpcode::Scan])
            .unwrap()
            .clone()
    }
}
