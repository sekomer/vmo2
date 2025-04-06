#[cfg(test)]
mod test {
    use core::panic;

    use crate::deserialize::deserialize;
    use crate::metadata::Version;
    use crate::serialize::serialize;
    use crate::traits::Serializable;
    use crate::v1::serialize::Serializer;

    #[test]
    fn test_non_literal_opcodes() {
        use vmo2_types::{ast::Ast, opcode::*};

        let ast = Ast::from(vec![
            Opcode::Halt,
            Opcode::Arithmetic(ArithmeticOpcode::Add),
            Opcode::Arithmetic(ArithmeticOpcode::Sub),
            Opcode::Arithmetic(ArithmeticOpcode::Mul),
            Opcode::Arithmetic(ArithmeticOpcode::Div),
            Opcode::Logic(LogicOpcode::And),
            Opcode::Logic(LogicOpcode::Xor),
            Opcode::Logic(LogicOpcode::Or),
            Opcode::Logic(LogicOpcode::Not),
            Opcode::Comparison(ComparisonOpcode::Eq),
            Opcode::Comparison(ComparisonOpcode::Ne),
            Opcode::Comparison(ComparisonOpcode::Lt),
            Opcode::Comparison(ComparisonOpcode::Le),
            Opcode::Comparison(ComparisonOpcode::Gt),
            Opcode::Comparison(ComparisonOpcode::Ge),
            Opcode::Memory(MemoryOpcode::Load),
            Opcode::Memory(MemoryOpcode::Store),
            Opcode::IO(IOOpcode::Print),
            Opcode::IO(IOOpcode::Scan),
        ]);

        let serializer = Serializer::new();
        let data = serializer.serialize(&ast);

        println!("{:?}", data);

        match deserialize(&data) {
            Ok(deserialized_ast) => assert_eq!(ast, deserialized_ast),
            Err(e) => panic!("failed to deserialize: {:?}", e),
        }
    }

    #[test]
    fn test_literal_opcodes() {
        use vmo2_types::{ast::Ast, opcode::*, value::Value};

        let ast = Ast::from(vec![
            Opcode::Literal(Value::Bool(true)),
            Opcode::Literal(Value::UInt(32)),
            Opcode::Literal(Value::String("abc".to_string())),
        ]);

        let data = serialize(Version::V1, &ast);

        println!("{:?}", data);

        match deserialize(&data) {
            Ok(deserialized_ast) => assert_eq!(ast, deserialized_ast),
            Err(e) => panic!("failed to deserialize: {:?}", e),
        }
    }

    #[test]
    fn test_quickcheck_v1() {
        /**
         * magic ✨
         */
        use vmo2_types::ast::Ast;

        fn test_ast(ast: Ast) -> bool {
            println!("ast: {:?}", ast);

            let data = serialize(Version::V1, &ast);
            if let Ok(deserialized_ast) = deserialize(&data) {
                ast == deserialized_ast
            } else {
                false
            }
        }

        quickcheck::quickcheck(test_ast as fn(Ast) -> bool);
    }
}
