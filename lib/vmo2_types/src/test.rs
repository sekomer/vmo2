#[cfg(test)]
mod tests {
    use crate::bytecode::ByteCode;
    use crate::opcode::*;
    use crate::value::Value;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_value_arithmetic() {
        assert_eq!(Value::UInt(5) + Value::UInt(3), Value::UInt(8));
        assert_eq!(
            Value::String("hello".to_string()) + Value::String(" world".to_string()),
            Value::String("hello world".to_string())
        );
        assert_eq!(Value::UInt(5) - Value::UInt(3), Value::UInt(2));
        assert_eq!(Value::UInt(5) * Value::UInt(3), Value::UInt(15));
        assert_eq!(Value::UInt(6) / Value::UInt(2), Value::UInt(3));
    }

    #[test]
    fn test_value_logic() {
        assert_eq!(!Value::Bool(true), Value::Bool(false));
        assert_eq!(!Value::Bool(false), Value::Bool(true));
        assert_eq!(Value::Bool(true).and(Value::Bool(true)), Value::Bool(true));
        assert_eq!(
            Value::Bool(true).and(Value::Bool(false)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::Bool(false).and(Value::Bool(false)),
            Value::Bool(false)
        );
        assert_eq!(Value::Bool(true).or(Value::Bool(true)), Value::Bool(true));
        assert_eq!(Value::Bool(true).or(Value::Bool(false)), Value::Bool(true));
        assert_eq!(
            Value::Bool(false).or(Value::Bool(false)),
            Value::Bool(false)
        );
        assert_eq!(Value::Bool(true).xor(Value::Bool(true)), Value::Bool(false));
        assert_eq!(Value::Bool(true).xor(Value::Bool(false)), Value::Bool(true));
        assert_eq!(
            Value::Bool(false).xor(Value::Bool(false)),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_bytecode_creation() {
        let mut bytecode = ByteCode::new();
        assert!(bytecode.opcodes.is_empty());

        bytecode.add_opcode(Opcode::Halt);
        assert_eq!(bytecode.opcodes.len(), 1);
        assert_eq!(bytecode.opcodes[0], Opcode::Halt);
    }

    #[test]
    fn test_bytecode_from_vec() {
        let opcodes = vec![Opcode::Halt, Opcode::Literal(Value::UInt(42))];
        let bytecode = ByteCode::from(opcodes.clone());
        assert_eq!(bytecode.opcodes, opcodes);
    }

    #[quickcheck]
    fn value_arbitrary_property(value: Value) -> bool {
        match value {
            Value::UInt(_) | Value::Bool(_) | Value::String(_) | Value::Null => true,
        }
    }

    #[quickcheck]
    fn opcode_arbitrary_property(opcode: Opcode) -> bool {
        match opcode {
            Opcode::Halt
            | Opcode::Literal(_)
            | Opcode::Arithmetic(_)
            | Opcode::Logic(_)
            | Opcode::Comparison(_)
            | Opcode::Memory(_)
            | Opcode::IO(_)
            | Opcode::Flow(_)
            | Opcode::Dup
            | Opcode::Pop
            | Opcode::Swap => true,
        }
    }

    #[quickcheck]
    fn bytecode_arbitrary_property(bytecode: ByteCode) -> bool {
        !bytecode.opcodes.is_empty() && bytecode.opcodes.last() == Some(&Opcode::Halt)
    }
}
