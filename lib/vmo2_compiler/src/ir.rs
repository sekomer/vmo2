use vmo2_types::value::Value;

use crate::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub instructions: Vec<IrInstruction>,
    pub next: Option<usize>,
    pub branch: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub parameters: Vec<String>,
    pub blocks: Vec<BasicBlock>,
    pub entry_block: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrInstruction {
    // Stack operations
    Push(Value),
    Pop,
    Dup,
    Swap,

    // Memory operations
    Load(String),
    Store(String),

    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,

    // Comparison operations
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,

    // Control flow
    Jump(usize),
    JumpIf(usize, usize),

    // Function operations
    Call(String),
    Return,

    // Other
    Print,
    NoOp,
    Neg,
}

#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: HashMap<String, IrFunction>,
    pub current_function: Option<String>,
    pub current_block: usize,
}

impl IrProgram {
    pub fn new() -> Self {
        IrProgram {
            functions: HashMap::new(),
            current_function: None,
            current_block: 0,
        }
    }

    pub fn add_function(&mut self, name: String, parameters: Vec<String>) {
        let entry_block = BasicBlock {
            instructions: Vec::new(),
            next: None,
            branch: None,
        };

        let function = IrFunction {
            name: name.clone(),
            parameters,
            blocks: vec![entry_block],
            entry_block: 0,
        };

        self.functions.insert(name.clone(), function);
        self.current_function = Some(name);
        self.current_block = 0;
    }

    pub fn add_instruction(&mut self, instruction: IrInstruction) {
        let function_name = self.current_function.as_ref().unwrap();
        let function = self.functions.get_mut(function_name).unwrap();
        let block = function.blocks.get_mut(self.current_block).unwrap();

        block.instructions.push(instruction);
    }

    pub fn add_block(&mut self) -> usize {
        if let Some(function_name) = &self.current_function {
            if let Some(function) = self.functions.get_mut(function_name) {
                let new_block = BasicBlock {
                    instructions: Vec::new(),
                    next: None,
                    branch: None,
                };
                let block_index = function.blocks.len();
                function.blocks.push(new_block);
                return block_index;
            }
        }
        0
    }

    pub fn link_blocks(&mut self, from: usize, to: usize) {
        if let Some(function_name) = &self.current_function {
            if let Some(function) = self.functions.get_mut(function_name) {
                if let Some(block) = function.blocks.get_mut(from) {
                    block.next = Some(to);
                }
            }
        }
    }
}

pub fn expression_to_ir(expression: &AstExpression) -> Vec<IrInstruction> {
    match expression {
        AstExpression::Literal(literal) => {
            let value = match literal {
                AstLiteral::UInt(n) => Value::UInt(*n),
                AstLiteral::String(s) => Value::String(s.clone()),
                AstLiteral::Bool(b) => Value::Bool(*b),
                AstLiteral::Null => Value::Null,
            };
            vec![IrInstruction::Push(value)]
        }
        AstExpression::Variable(name) => {
            vec![IrInstruction::Load(name.clone())]
        }
        AstExpression::UnaryOperation(op, expr) => {
            let mut instructions = Vec::new();
            instructions.extend(expression_to_ir(expr));
            let op_instruction = match op.as_str() {
                "-" => IrInstruction::Neg,
                _ => unreachable!(),
            };
            instructions.push(op_instruction);
            instructions
        }
        AstExpression::BinaryOperation(op, left, right) => {
            let mut instructions = Vec::new();
            instructions.extend(expression_to_ir(right));
            instructions.extend(expression_to_ir(left));

            let op_instruction = match op.as_str() {
                "+" => IrInstruction::Add,
                "-" => IrInstruction::Sub,
                "*" => IrInstruction::Mul,
                "/" => IrInstruction::Div,
                "==" => IrInstruction::Eq,
                "!=" => IrInstruction::Ne,
                "<" => IrInstruction::Lt,
                ">" => IrInstruction::Gt,
                "<=" => IrInstruction::Le,
                ">=" => IrInstruction::Ge,
                _ => unreachable!(),
            };

            instructions.push(op_instruction);
            instructions
        }
        AstExpression::FunctionCall(name, args) => {
            let mut instructions = Vec::new();
            for arg in args {
                instructions.extend(expression_to_ir(arg));
            }
            instructions.push(IrInstruction::Call(name.clone()));
            instructions
        }
    }
}
