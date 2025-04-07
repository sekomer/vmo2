use vmo2_types::{ast::*, opcode::*, value::*};
use vmo2_vm::vm::VM;

fn main() {
    // random program
    let mut ast = Ast::new();

    /*
       n = 0
       while n < 10 {
           print n
           n++
       }
    */

    // n = 0
    ast.add_opcode(Opcode::Literal(Value::UInt(0)));
    ast.add_opcode(Opcode::Literal(Value::String("n".to_string())));
    ast.add_opcode(Opcode::Memory(MemoryOpcode::Store));

    // while loop
    ast.add_opcode(Opcode::Literal(Value::String("n".to_string())));
    ast.add_opcode(Opcode::Memory(MemoryOpcode::Load));
    ast.add_opcode(Opcode::Dup);

    ast.add_opcode(Opcode::Literal(Value::UInt(10)));
    ast.add_opcode(Opcode::Comparison(ComparisonOpcode::Le));

    ast.add_opcode(Opcode::Flow(FlowOpcode::JumpIfTrue(17)));

    ast.add_opcode(Opcode::Dup);
    ast.add_opcode(Opcode::IO(IOOpcode::Print));

    // n++ (using n thats duplicated before)
    ast.add_opcode(Opcode::Literal(Value::UInt(1)));
    ast.add_opcode(Opcode::Arithmetic(ArithmeticOpcode::Add));

    // store n
    ast.add_opcode(Opcode::Literal(Value::String("n".to_string())));
    ast.add_opcode(Opcode::Memory(MemoryOpcode::Store));

    ast.add_opcode(Opcode::Flow(FlowOpcode::Jump(3)));
    // end of while loop

    // pop unused
    ast.add_opcode(Opcode::Pop);

    ast.add_opcode(Opcode::Halt);

    let mut vm = VM::new(ast);
    let profile = vm.run().unwrap();
    println!("Profile: {:?}", profile);
}
