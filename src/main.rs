use vmo2_types::{ast, opcode::Opcode::*, opcode::*, value::Value};
use vmo2_vm::vm::VM;

fn main() {
    // random program
    let mut ast = ast::Ast::new();

    ast.add_opcode(Literal(Value::String("Hello, world!".to_owned())));
    ast.add_opcode(Literal(Value::String("Enter your name: ".to_owned())));
    ast.add_opcode(IO(IOOpcode::Print));
    ast.add_opcode(IO(IOOpcode::Scan));

    ast.add_opcode(Literal(Value::String("Hello, ".to_owned())));
    ast.add_opcode(Arithmetic(ArithmeticOpcode::Add));

    ast.add_opcode(IO(IOOpcode::Print));

    let mut vm = VM::new(ast);
    let profile = vm.run().unwrap();
    println!("Profile: {:?}", profile);
}
