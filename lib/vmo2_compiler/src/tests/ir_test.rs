#[cfg(test)]
mod tests {
    use crate::ir::IrInstruction;
    use crate::ir_compiler::*;
    use crate::parser::*;
    use crate::types::*;
    use pest::Parser;
    use vmo2_types::opcode::Opcode;
    use vmo2_types::{opcode::MemoryOpcode, value::Value};

    #[test]
    fn test_assignment() {
        let code = r#"x = 1;"#;

        let program_pair = OxydeParser::parse(Rule::program, code)
            .unwrap()
            .next()
            .unwrap();

        let program = parse_program(program_pair);

        let ir = compile_to_ir(program);
        let bytecode = ir_to_bytecode(ir.clone());

        assert_eq!(ir.functions.len(), 1);
        assert_eq!(ir.functions["main"].blocks.len(), 1);
        assert_eq!(ir.functions["main"].blocks[0].instructions.len(), 2);
        assert_eq!(
            ir.functions["main"].blocks[0].instructions,
            vec![
                IrInstruction::Push(Value::UInt(1)),
                IrInstruction::Store("x".to_string())
            ]
        );

        assert_eq!(bytecode.opcodes.len(), 3);
        assert_eq!(
            bytecode.opcodes,
            vec![
                Opcode::Literal(Value::UInt(1)),
                Opcode::Literal(Value::String("x".to_string())),
                Opcode::Memory(MemoryOpcode::Store),
            ]
        );
    }

    #[test]
    fn test_multi_assignment() {
        let code = r#"
            x = 1;
            y = 2;
            z = x;
        "#;

        let program_pair = OxydeParser::parse(Rule::program, code)
            .unwrap()
            .next()
            .unwrap();

        let program = parse_program(program_pair);
        let ir = compile_to_ir(program);
        let bytecode = ir_to_bytecode(ir.clone());

        assert_eq!(ir.functions.len(), 1);
        assert_eq!(ir.functions["main"].blocks.len(), 1);
        assert_eq!(ir.functions["main"].blocks[0].instructions.len(), 6);
        assert_eq!(
            ir.functions["main"].blocks[0].instructions,
            vec![
                IrInstruction::Push(Value::UInt(1)),
                IrInstruction::Store("x".to_string()),
                IrInstruction::Push(Value::UInt(2)),
                IrInstruction::Store("y".to_string()),
                IrInstruction::Load("x".to_string()),
                IrInstruction::Store("z".to_string()),
            ]
        );

        assert_eq!(
            bytecode.opcodes,
            vec![
                Opcode::Literal(Value::UInt(1)),
                Opcode::Literal(Value::String("x".to_string())),
                Opcode::Memory(MemoryOpcode::Store),
                Opcode::Literal(Value::UInt(2)),
                Opcode::Literal(Value::String("y".to_string())),
                Opcode::Memory(MemoryOpcode::Store),
                Opcode::Literal(Value::String("x".to_string())),
                Opcode::Memory(MemoryOpcode::Load),
                Opcode::Literal(Value::String("z".to_string())),
                Opcode::Memory(MemoryOpcode::Store),
            ]
        );
    }
}
