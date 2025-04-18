use crate::metadata::MAGIC;
use crate::traits::DeserializationError;
use crate::{traits::Deserializable, v1::deserialize::Deserializer as V1Deserializer};
use vmo2_types::bytecode;

pub fn deserialize(input: &Vec<u8>) -> Result<bytecode::ByteCode, DeserializationError> {
    let magic_number: u32 = u32::from_le_bytes(input[0..4].try_into().unwrap());
    if magic_number != MAGIC {
        return Err(DeserializationError::InvalidMagicNumber);
    }

    let version: u8 = input[4].try_into().unwrap();
    let code = Vec::from(&input[5..]);

    match version {
        1 => V1Deserializer::new().deserialize(&code),
        2 => todo!(),
        _ => Err(DeserializationError::InvalidVersion),
    }
}
