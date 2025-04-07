use vmo2_types::bytecode;

pub trait Serializable {
    type Output;

    fn serialize(&self, bytecode: &bytecode::ByteCode) -> Self::Output;
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeserializationError {
    InvalidMagicNumber,
    InvalidVersion,
    InvalidByteCode,
}

pub trait Deserializable {
    fn deserialize(&self, input: &Vec<u8>) -> Result<bytecode::ByteCode, DeserializationError>;
}
