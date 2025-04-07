use crate::{metadata::Version, traits::Serializable, v1::serialize::Serializer as V1Serializer};
use vmo2_types::bytecode;

pub fn serialize(version: u8, bytecode: &bytecode::ByteCode) -> Vec<u8> {
    match version {
        Version::V1 => V1Serializer::new().serialize(bytecode),
        Version::V2 => todo!(),
        _ => panic!("Unsupported version"),
    }
}
