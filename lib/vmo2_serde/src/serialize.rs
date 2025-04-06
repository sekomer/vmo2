use crate::{traits::Serializable, v1::serialize::Serializer as V1Serializer};
use vmo2_types::ast;

pub fn serialize(version: u32, ast: &ast::Ast) -> Vec<u8> {
    match version {
        1 => V1Serializer::new().serialize(ast),
        2 => todo!(),
        _ => panic!("Unsupported version"),
    }
}
