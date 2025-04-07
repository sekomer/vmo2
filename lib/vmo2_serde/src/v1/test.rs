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
        use vmo2_types::{bytecode::ByteCode, opcode::*};

        let bytecode = ByteCode::from(vec![
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
        let data = serializer.serialize(&bytecode);

        println!("{:?}", data);

        match deserialize(&data) {
            Ok(deserialized_bytecode) => assert_eq!(bytecode, deserialized_bytecode),
            Err(e) => panic!("failed to deserialize: {:?}", e),
        }
    }

    #[test]
    fn test_literal_opcodes() {
        use vmo2_types::{bytecode::ByteCode, opcode::*, value::Value};

        let bytecode = ByteCode::from(vec![
            Opcode::Literal(Value::Bool(true)),
            Opcode::Literal(Value::UInt(32)),
            Opcode::Literal(Value::String("abc".to_string())),
        ]);

        let data = serialize(Version::V1, &bytecode);

        println!("{:?}", data);

        match deserialize(&data) {
            Ok(deseri) => assert_eq!(bytecode, deseri),
            Err(e) => panic!("failed to deserialize: {:?}", e),
        }
    }

    #[test]
    fn test_quickcheck_v1() {
        /**
         * magic ✨
         */
        use vmo2_types::bytecode::ByteCode;

        fn test_bytecode(bytecode: ByteCode) -> bool {
            println!("bytecode: {:?}", bytecode);

            let data = serialize(Version::V1, &bytecode);
            if let Ok(deseri) = deserialize(&data) {
                bytecode == deseri
            } else {
                false
            }
        }

        quickcheck::quickcheck(test_bytecode as fn(ByteCode) -> bool);
    }
}
