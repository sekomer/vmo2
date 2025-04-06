use vmo2_types::ast;

pub trait Serializable {
    type Output;

    fn serialize(&self, ast: &ast::Ast) -> Self::Output;
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeserializationError {
    InvalidMagicNumber,
    InvalidVersion,
    InvalidAst,
}

pub trait Deserializable {
    fn deserialize(&self, input: &Vec<u8>) -> Result<ast::Ast, DeserializationError>;
}
