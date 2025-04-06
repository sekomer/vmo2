use crate::profile;
use std::collections::HashMap;

use vmo2_types::{ast, opcode, value};

pub struct VM {
    pub stack: Vec<value::Value>,
    pub heap: HashMap<String, value::Value>,
    pub pc: usize,
    pub ast: ast::Ast,
    pub debug: bool,
    pub profile: profile::Profile,
}

#[derive(Debug)]
pub enum VMResult {
    Ok,
    Error(VMError),
    Halt,
}

#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    HeapUnderflow,
    InvalidOpcode,
    InvalidOpcodeArgument,
}

impl VM {
    pub fn new(ast: ast::Ast) -> Self {
        Self {
            ast,
            stack: vec![],
            heap: HashMap::new(),
            pc: 0,
            debug: false,
            profile: profile::Profile::new(),
        }
    }

    pub fn run(&mut self) -> Result<profile::Profile, VMError> {
        loop {
            match self.step() {
                VMResult::Ok => continue,
                VMResult::Error(e) => return Err(e),
                VMResult::Halt => break,
            }
        }

        Ok(self.profile.clone())
    }

    fn step(&mut self) -> VMResult {
        let opcode = self.ast.opcodes[self.pc].clone();
        self.pc += 1;
        self.profile.total_instructions += 1;

        use opcode::Opcode::*;
        match opcode {
            Literal(value) => {
                self.stack.push(value);
                self.profile.total_stack_pushes += 1;
                return VMResult::Ok;
            }
            Arithmetic(arithmetic) => {
                use opcode::ArithmeticOpcode::*;
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.profile.total_stack_pops += 2;
                let result = match arithmetic {
                    Add => a + b,
                    Sub => a - b,
                    Mul => a * b,
                    Div => a / b,
                };
                self.stack.push(result);
                self.profile.total_stack_pushes += 1;
                return VMResult::Ok;
            }
            Logic(logic) => {
                use opcode::LogicOpcode::*;
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.profile.total_stack_pops += 2;
                let result = match logic {
                    And => a.and(b),
                    Or => a.or(b),
                    Xor => a.xor(b),
                    Not => !a,
                };
                self.stack.push(result);
                self.profile.total_stack_pushes += 1;
                return VMResult::Ok;
            }
            Comparison(comparison) => {
                use opcode::ComparisonOpcode::*;
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.profile.total_stack_pops += 2;
                let result = match comparison {
                    Eq => a == b,
                    Ne => a != b,
                    Le => a <= b,
                    Lt => a < b,
                    Gt => a > b,
                    Ge => a >= b,
                };
                self.stack.push(value::Value::Bool(result));
                self.profile.total_stack_pushes += 1;
                return VMResult::Ok;
            }
            Memory(memory) => {
                use opcode::MemoryOpcode::*;
                match memory {
                    Load => {
                        let value::Value::String(key) = self.stack.pop().unwrap() else {
                            return VMResult::Error(VMError::InvalidOpcodeArgument);
                        };
                        self.profile.total_stack_pops += 1;
                        let value = self.heap.get(&key).unwrap();
                        self.profile.total_memory_reads += 1;
                        self.stack.push(value.clone());
                        self.profile.total_stack_pushes += 1;
                        return VMResult::Ok;
                    }
                    Store => {
                        let value::Value::String(key) = self.stack.pop().unwrap() else {
                            return VMResult::Error(VMError::InvalidOpcodeArgument);
                        };
                        let value = self.stack.pop().unwrap();
                        self.profile.total_stack_pops += 2;
                        self.heap.insert(key, value);
                        self.profile.total_memory_writes += 1;
                        return VMResult::Ok;
                    }
                }
            }
            IO(io) => {
                use opcode::IOOpcode::*;
                match io {
                    Print => {
                        let value = self.stack.pop().unwrap();
                        self.profile.total_stack_pops += 1;
                        println!("{:?}", value);
                        return VMResult::Ok;
                    }
                    Scan => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        self.stack
                            .push(value::Value::String(input.trim().to_string()));
                        self.profile.total_stack_pushes += 1;
                        return VMResult::Ok;
                    }
                }
            }
            Halt => {
                return VMResult::Halt;
            }
        }
    }
}
