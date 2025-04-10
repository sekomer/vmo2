use crate::ir::*;
use crate::ir_builder::*;
use crate::types::*;
use vmo2_types::bytecode::*;
use vmo2_types::opcode::*;
use vmo2_types::value::Value;

pub fn compile_to_ir(program: AstProgram) -> IrProgram {
    let mut ir = IrProgram::new();
    let mut builder = IrBuilder::new(&mut ir, "main");

    for statement in program.statements {
        builder.emit_stmt(&statement);
    }

    ir
}

pub fn ir_to_bytecode(ir: IrProgram) -> ByteCode {
    let mut bytecode = ByteCode::new();

    for function in ir.functions.values() {
        for block in &function.blocks {
            for instr in &block.instructions {
                match instr {
                    IrInstruction::Push(val) => {
                        bytecode.add_opcode(Opcode::Literal(val.clone()));
                    }
                    IrInstruction::Pop => {
                        bytecode.add_opcode(Opcode::Pop);
                    }
                    IrInstruction::Dup => {
                        bytecode.add_opcode(Opcode::Dup);
                    }
                    IrInstruction::Swap => {
                        bytecode.add_opcode(Opcode::Swap);
                    }
                    IrInstruction::Load(name) => {
                        bytecode.add_opcode(Opcode::Literal(Value::String(name.clone())));
                        bytecode.add_opcode(Opcode::Memory(MemoryOpcode::Load));
                    }
                    IrInstruction::Store(name) => {
                        bytecode.add_opcode(Opcode::Literal(Value::String(name.clone())));
                        bytecode.add_opcode(Opcode::Memory(MemoryOpcode::Store));
                    }
                    IrInstruction::Add => {
                        bytecode.add_opcode(Opcode::Arithmetic(ArithmeticOpcode::Add));
                    }
                    IrInstruction::Sub => {
                        bytecode.add_opcode(Opcode::Arithmetic(ArithmeticOpcode::Sub));
                    }
                    IrInstruction::Mul => {
                        bytecode.add_opcode(Opcode::Arithmetic(ArithmeticOpcode::Mul));
                    }
                    IrInstruction::Div => {
                        bytecode.add_opcode(Opcode::Arithmetic(ArithmeticOpcode::Div));
                    }
                    IrInstruction::Eq => {
                        bytecode.add_opcode(Opcode::Comparison(ComparisonOpcode::Eq));
                    }
                    IrInstruction::Ne => {
                        bytecode.add_opcode(Opcode::Comparison(ComparisonOpcode::Ne));
                    }
                    IrInstruction::Lt => {
                        bytecode.add_opcode(Opcode::Comparison(ComparisonOpcode::Lt));
                    }
                    IrInstruction::Gt => {
                        bytecode.add_opcode(Opcode::Comparison(ComparisonOpcode::Gt));
                    }
                    IrInstruction::Le => {
                        bytecode.add_opcode(Opcode::Comparison(ComparisonOpcode::Le));
                    }
                    IrInstruction::Ge => {
                        bytecode.add_opcode(Opcode::Comparison(ComparisonOpcode::Ge));
                    }
                    IrInstruction::Print => {
                        bytecode.add_opcode(Opcode::IO(IOOpcode::Print));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    bytecode
}
