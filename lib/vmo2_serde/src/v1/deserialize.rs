use crate::traits::Deserializable;
use crate::traits::DeserializationError;
use crate::v1::constants::OPCODE;
use nom::bytes;
use nom::character;
use nom::{
    IResult, branch,
    bytes::complete::tag,
    combinator,
    multi::{length_count, many1},
    sequence,
};
use vmo2_types::ast;
use vmo2_types::opcode::*;
use vmo2_types::value::Value;

pub struct Deserializer {}

impl Deserializer {
    pub fn new() -> Self {
        Self {}
    }
}

fn parser(input: &[u8]) -> IResult<&[u8], Opcode> {
    let halt_parser = combinator::value(Opcode::Halt, tag([OPCODE::HALT]));

    let int_literal_parser = sequence::preceded(
        tag([OPCODE::LITERAL_UINT]),
        combinator::map(bytes::complete::take(4u8), |bytes: &[u8]| {
            Opcode::Literal(Value::UInt(u32::from_le_bytes(bytes.try_into().unwrap())))
        }),
    );
    let bool_literal_parser = sequence::preceded(
        tag([OPCODE::LITERAL_BOOL]),
        branch::alt((
            combinator::value(Opcode::Literal(Value::Bool(true)), tag([1])),
            combinator::value(Opcode::Literal(Value::Bool(false)), tag([0])),
        )),
    );
    let string_literal_parser = sequence::preceded(
        tag([OPCODE::LITERAL_STRING]),
        combinator::map(
            length_count(
                combinator::map(bytes::complete::take(2u8), |bytes: &[u8]| {
                    u16::from_le_bytes(bytes.try_into().unwrap())
                }),
                character::complete::anychar,
            ),
            |s| {
                let vec_u8 = s.iter().map(|c| *c as u8).collect::<Vec<_>>();
                Opcode::Literal(Value::String(String::from_utf8(vec_u8).unwrap()))
            },
        ),
    );
    let literal_parser = sequence::preceded(
        tag([OPCODE::LITERAL]),
        branch::alt((
            bool_literal_parser,
            int_literal_parser,
            string_literal_parser,
        )),
    );

    let arithmetic_parser = sequence::preceded(
        tag([OPCODE::ARITHMETIC]),
        branch::alt((
            combinator::value(
                Opcode::Arithmetic(ArithmeticOpcode::Add),
                tag([OPCODE::ARITHMETIC_ADD]),
            ),
            combinator::value(
                Opcode::Arithmetic(ArithmeticOpcode::Sub),
                tag([OPCODE::ARITHMETIC_SUB]),
            ),
            combinator::value(
                Opcode::Arithmetic(ArithmeticOpcode::Mul),
                tag([OPCODE::ARITHMETIC_MUL]),
            ),
            combinator::value(
                Opcode::Arithmetic(ArithmeticOpcode::Div),
                tag([OPCODE::ARITHMETIC_DIV]),
            ),
        )),
    );

    let logic_parser = sequence::preceded(
        tag([OPCODE::LOGIC]),
        branch::alt((
            combinator::value(Opcode::Logic(LogicOpcode::And), tag([OPCODE::LOGIC_AND])),
            combinator::value(Opcode::Logic(LogicOpcode::Or), tag([OPCODE::LOGIC_OR])),
            combinator::value(Opcode::Logic(LogicOpcode::Xor), tag([OPCODE::LOGIC_XOR])),
            combinator::value(Opcode::Logic(LogicOpcode::Not), tag([OPCODE::LOGIC_NOT])),
        )),
    );

    let comparison_parser = sequence::preceded(
        tag([OPCODE::COMPARISON]),
        branch::alt((
            combinator::value(
                Opcode::Comparison(ComparisonOpcode::Eq),
                tag([OPCODE::COMPARISON_EQ]),
            ),
            combinator::value(
                Opcode::Comparison(ComparisonOpcode::Ne),
                tag([OPCODE::COMPARISON_NE]),
            ),
            combinator::value(
                Opcode::Comparison(ComparisonOpcode::Lt),
                tag([OPCODE::COMPARISON_LT]),
            ),
            combinator::value(
                Opcode::Comparison(ComparisonOpcode::Le),
                tag([OPCODE::COMPARISON_LE]),
            ),
            combinator::value(
                Opcode::Comparison(ComparisonOpcode::Gt),
                tag([OPCODE::COMPARISON_GT]),
            ),
            combinator::value(
                Opcode::Comparison(ComparisonOpcode::Ge),
                tag([OPCODE::COMPARISON_GE]),
            ),
        )),
    );

    let memory_parser = sequence::preceded(
        tag([OPCODE::MEMORY]),
        branch::alt((
            combinator::value(
                Opcode::Memory(MemoryOpcode::Load),
                tag([OPCODE::MEMORY_LOAD]),
            ),
            combinator::value(
                Opcode::Memory(MemoryOpcode::Store),
                tag([OPCODE::MEMORY_STORE]),
            ),
        )),
    );

    let io_parser = sequence::preceded(
        tag([OPCODE::IO]),
        branch::alt((
            combinator::value(Opcode::IO(IOOpcode::Print), tag([OPCODE::IO_PRINT])),
            combinator::value(Opcode::IO(IOOpcode::Scan), tag([OPCODE::IO_SCAN])),
        )),
    );

    branch::alt((
        halt_parser,
        arithmetic_parser,
        logic_parser,
        comparison_parser,
        memory_parser,
        io_parser,
        literal_parser,
    ))(input)
}

impl Deserializable for Deserializer {
    fn deserialize(&self, input: &Vec<u8>) -> Result<ast::Ast, DeserializationError> {
        match many1(parser)(input) {
            Ok((_, opcode)) => Ok(ast::Ast::from(opcode)),
            Err(e) => {
                eprintln!("error: {:?}", e);
                Err(DeserializationError::InvalidAst)
            }
        }
    }
}
