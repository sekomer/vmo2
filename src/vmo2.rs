use vmo2_types::{bytecode::*, opcode::*, value::*};
use vmo2_vm::vm::VM;

fn main() {
    // random program
    let mut bytecode = ByteCode::new();

    /*
       n = 0
       while n < 10 {
           print n
           n++
       }
    */

    // n = 0
    bytecode.add_opcode(Opcode::Literal(Value::UInt(0)));
    bytecode.add_opcode(Opcode::Literal(Value::String("n".to_string())));
    bytecode.add_opcode(Opcode::Memory(MemoryOpcode::Store));

    // while loop
    bytecode.add_opcode(Opcode::Literal(Value::String("n".to_string())));
    bytecode.add_opcode(Opcode::Memory(MemoryOpcode::Load));
    bytecode.add_opcode(Opcode::Dup);

    bytecode.add_opcode(Opcode::Literal(Value::UInt(10)));
    bytecode.add_opcode(Opcode::Comparison(ComparisonOpcode::Le));

    bytecode.add_opcode(Opcode::Flow(FlowOpcode::JumpIfTrue(17)));

    bytecode.add_opcode(Opcode::Dup);
    bytecode.add_opcode(Opcode::IO(IOOpcode::Print));

    // n++ (using n thats duplicated before)
    bytecode.add_opcode(Opcode::Literal(Value::UInt(1)));
    bytecode.add_opcode(Opcode::Arithmetic(ArithmeticOpcode::Add));

    // store n
    bytecode.add_opcode(Opcode::Literal(Value::String("n".to_string())));
    bytecode.add_opcode(Opcode::Memory(MemoryOpcode::Store));

    bytecode.add_opcode(Opcode::Flow(FlowOpcode::Jump(3)));
    // end of while loop

    // pop unused
    bytecode.add_opcode(Opcode::Pop);

    bytecode.add_opcode(Opcode::Halt);

    let mut vm = VM::new(bytecode);
    let profile = vm.run().unwrap();
    println!("Profile: {:?}", profile);
}
