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
    // * HALT
    let halt_parser = combinator::value(Opcode::Halt, tag([OPCODE::HALT]));

    // * LITERAL
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

    // * ARITHMETIC
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

    // * LOGIC
    let logic_parser = sequence::preceded(
        tag([OPCODE::LOGIC]),
        branch::alt((
            combinator::value(Opcode::Logic(LogicOpcode::And), tag([OPCODE::LOGIC_AND])),
            combinator::value(Opcode::Logic(LogicOpcode::Or), tag([OPCODE::LOGIC_OR])),
            combinator::value(Opcode::Logic(LogicOpcode::Xor), tag([OPCODE::LOGIC_XOR])),
            combinator::value(Opcode::Logic(LogicOpcode::Not), tag([OPCODE::LOGIC_NOT])),
        )),
    );

    // * COMPARISION
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

    // * MEMORY
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

    // * IO
    let io_parser = sequence::preceded(
        tag([OPCODE::IO]),
        branch::alt((
            combinator::value(Opcode::IO(IOOpcode::Print), tag([OPCODE::IO_PRINT])),
            combinator::value(Opcode::IO(IOOpcode::Scan), tag([OPCODE::IO_SCAN])),
        )),
    );

    // * FLOW
    let flow_jump_if_true_parser = sequence::preceded(
        tag([OPCODE::FLOW_JUMP_IF_TRUE]),
        combinator::map(bytes::complete::take(4u8), |bytes: &[u8]| {
            Opcode::Flow(FlowOpcode::JumpIfTrue(u32::from_le_bytes(
                bytes.try_into().unwrap(),
            )))
        }),
    );
    let flow_jump_if_false_parser = sequence::preceded(
        tag([OPCODE::FLOW_JUMP_IF_FALSE]),
        combinator::map(bytes::complete::take(4u8), |bytes: &[u8]| {
            Opcode::Flow(FlowOpcode::JumpIfFalse(u32::from_le_bytes(
                bytes.try_into().unwrap(),
            )))
        }),
    );
    let flow_jump_parser = sequence::preceded(
        tag([OPCODE::FLOW_JUMP]),
        combinator::map(bytes::complete::take(4u8), |bytes: &[u8]| {
            Opcode::Flow(FlowOpcode::Jump(u32::from_le_bytes(
                bytes.try_into().unwrap(),
            )))
        }),
    );
    let flow_call_parser = sequence::preceded(
        tag([OPCODE::FLOW_CALL]),
        combinator::map(bytes::complete::take(4u8), |bytes: &[u8]| {
            Opcode::Flow(FlowOpcode::Call(u32::from_le_bytes(
                bytes.try_into().unwrap(),
            )))
        }),
    );
    let flow_return_parser =
        combinator::value(Opcode::Flow(FlowOpcode::Return), tag([OPCODE::FLOW_RETURN]));
    let flow_parser = sequence::preceded(
        tag([OPCODE::FLOW]),
        branch::alt((
            flow_jump_if_false_parser,
            flow_jump_if_true_parser,
            flow_jump_parser,
            flow_call_parser,
            flow_return_parser,
        )),
    );

    // * DUP
    let dup_parser = combinator::value(Opcode::Dup, tag([OPCODE::DUP]));

    // * POP
    let pop_parser = combinator::value(Opcode::Pop, tag([OPCODE::POP]));

    // * SWAP
    let swap_parser = combinator::value(Opcode::Swap, tag([OPCODE::SWAP]));

    branch::alt((
        halt_parser,
        arithmetic_parser,
        logic_parser,
        comparison_parser,
        memory_parser,
        io_parser,
        literal_parser,
        flow_parser,
        dup_parser,
        pop_parser,
        swap_parser,
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
