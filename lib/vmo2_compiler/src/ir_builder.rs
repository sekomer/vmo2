use crate::ir::*;
use crate::types::*;

pub struct IrBuilder<'a> {
    ir: &'a mut IrProgram,
    current_fn: String,
}

impl<'a> IrBuilder<'a> {
    pub fn new(ir: &'a mut IrProgram, function_name: &str) -> Self {
        ir.add_function(function_name.to_string(), vec![]);
        Self {
            ir,
            current_fn: function_name.to_string(),
        }
    }

    pub fn emit_expr(&mut self, expr: &AstExpression) {
        let instructions = expression_to_ir(expr);
        for instr in instructions {
            self.ir.add_instruction(instr);
        }
    }

    pub fn emit_stmt(&mut self, stmt: &AstStatement) {
        match stmt {
            AstStatement::Assignment(name, expr) => {
                self.emit_expr(expr);
                self.ir.add_instruction(IrInstruction::Store(name.clone()));
            }
            AstStatement::Expression(expr) => {
                self.emit_expr(expr);
            }
            AstStatement::While(cond, body) => {
                /*
                             +--------------------+
                             |    Entry Block     |
                             | Jump to cond_block |
                             +---------+----------+
                                       |
                                       v
                              +--------+--------+
                              |  cond_block     |
                              | evaluate `cond` |
                              +--------+--------+
                                       |
                             +---------+---------+
                             |                   |
                             v                   v
                    +--------+--------+   +------+------+
                    | body_block      |   | after_block |
                    | emit body stmts |   | (exit loop) |
                    | jump to cond    |   +-------------+
                    +--------+--------+
                             |
                             v
                    (back to cond_block)
                */
                let cond_block = self.ir.add_block();
                let body_block = self.ir.add_block();
                let after_block = self.ir.add_block();

                self.ir.add_instruction(IrInstruction::Jump(cond_block));

                self.ir.current_block = cond_block;
                self.emit_expr(cond);
                self.ir
                    .add_instruction(IrInstruction::JumpIf(body_block, after_block));

                self.ir.current_block = body_block;
                for stmt in body {
                    self.emit_stmt(stmt);
                }
                self.ir.add_instruction(IrInstruction::Jump(cond_block));

                self.ir.current_block = after_block;
            }
            AstStatement::FunctionDefinition(name, body) => {
                let old_function = self.current_fn.clone();
                let old_block = self.ir.current_block;

                self.ir.add_function(name.clone(), vec![]);
                self.current_fn = name.clone();
                self.ir.current_block = 0;

                for stmt in body {
                    self.emit_stmt(stmt);
                }

                self.ir.add_instruction(IrInstruction::Return);

                self.current_fn = old_function;
                self.ir.current_block = old_block;
            }
        }
    }
}
