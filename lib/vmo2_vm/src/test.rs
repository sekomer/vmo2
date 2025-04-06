#[cfg(test)]
mod tests {
    #[test]
    fn test_arithmetic_with_memory() {
        use crate::vm::VM;
        use vmo2_types::{
            ast::Ast,
            opcode::{ArithmeticOpcode, MemoryOpcode, Opcode::*},
            value::Value,
        };

        let ast = Ast::from(vec![
            // Store x = 3
            Literal(Value::UInt(3)),
            Literal(Value::String("x".to_owned())),
            Memory(MemoryOpcode::Store),
            // Store y = 4
            Literal(Value::UInt(4)),
            Literal(Value::String("y".to_owned())),
            Memory(MemoryOpcode::Store),
            // Load x
            Literal(Value::String("x".to_owned())),
            Memory(MemoryOpcode::Load),
            // Load y
            Literal(Value::String("y".to_owned())),
            Memory(MemoryOpcode::Load),
            // Add
            Arithmetic(ArithmeticOpcode::Add),
            Halt,
        ]);

        let mut vm = VM::new(ast);
        _ = vm.run().unwrap();

        assert_eq!(vm.stack.pop().unwrap(), Value::UInt(7));
    }
}
