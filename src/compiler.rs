use std::collections::HashMap;

use crate::statement::*;
use crate::encode::*;
use crate::AstNode;

pub struct Compiler {
    output: String,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            output: String::new(),
        }
    }

    pub fn compile(&mut self, ast: AstNode) {
        match ast {
            AstNode::Program(statements) => {
                for statement in statements {
                    self.compile_statement(statement);
                }
            },

            _ => panic!("214"),
        }
    }

    fn compile_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Function(func) => self.compile_function(func),
            Statement::VariableDeclaration(var_decl) => self.compile_variable_declaration(var_decl),
            Statement::If(if_stmt) => self.compile_if_statement(if_stmt),
            Statement::While(while_stmt) => self.compile_while_statement(while_stmt),
            Statement::For(for_stmt) => self.compile_for_statement(for_stmt),

            _ => unimplemented!(),
        }
    }

    fn compile_function(&mut self, func: FunctionDef) {
        self.output.push_str(&format!("{}:\n", func.name));
        self.output.push_str("    ; Function prologue\n");
        self.output.push_str("    push %rbp\n");
        self.output.push_str("    mov %rsp, %rbp\n");

        for stmt in func.body {
            self.compile_statement(stmt);
        }

        self.output.push_str("    ; Function epilogue\n");
        self.output.push_str("    pop %rbp\n");
        self.output.push_str("    ret\n");
    }

    fn compile_variable_declaration(&mut self, var_decl: VariableDeclaration) {
        let return_str;

        if var_decl.var_type.chars()
                            .next()
                            .unwrap() 
            == '%' 
        {
            return_str = var_decl.var_type;
        } else {
            return_str = format!("[{}]", var_decl.name);
        }

        match *var_decl.value {
            Expression::Number(n) => {
                self.output.push_str(&format!("    mov ${}, {}\n", n, &return_str));
            },

            Expression::Identifier(ref name) => {
                self.output.push_str(&format!("    mov %{}, {}\n", name, &return_str));
            },

            Expression::BinaryOperation { ref left, ref operator, ref right } => {
                // Compile the left expression
                expression(left, &mut self.output);
                self.output.push_str("    push %rax\n");
                
                // Compile the right expression
                expression(right, &mut self.output);
                self.output.push_str("    pop %rbx\n");

                // Generate code based on the operator
                match operator.as_str() {
                    "+" => self.output.push_str("    add %rbx, %rax\n"),
                    "-" => self.output.push_str("    sub %rbx, %rax\n"),
                    "*" => self.output.push_str("    imul %rbx, %rax\n"),
                    "/" => {
                        self.output.push_str("    cqo\n");
                        self.output.push_str("    idiv %rbx\n");
                    },

                    // Add additional operators as needed
                    _ => unimplemented!("Operator not implemented: {}", operator),
                }

                self.output.push_str(&format!("    mov %rax, %{}\n", var_decl.name));
            },

           // _ => unimplemented!("Variable declaration type not implemented"),
        }
    }

    fn compile_if_statement(&mut self, if_stmt: IfStatement) {
        self.output.push_str("    ; If statement\n");

        // Assume condition generates a label for branching
        expression(&if_stmt.condition, &mut self.output);
        self.output.push_str("    jmp .if_end\n");

        for stmt in if_stmt.then_branch {
            self.compile_statement(stmt);
        }

        if let Some(else_branch) = if_stmt.else_branch {
            self.output.push_str("    jmp .else_end\n");
            self.output.push_str(".if_end:\n");
            for stmt in else_branch {
                self.compile_statement(stmt);
            }
        }

        self.output.push_str(".else_end:\n");
    }

    fn compile_while_statement(&mut self, while_stmt: WhileStatement) {
        self.output.push_str(".while_start:\n");
        expression(&while_stmt.condition, &mut self.output);
        self.output.push_str("    jmp .while_end\n");

        for stmt in while_stmt.body {
            self.compile_statement(stmt);
        }

        self.output.push_str(".while_end:\n");
    }

    fn compile_for_statement(&mut self, for_stmt: ForStatement) {
        self.output.push_str(&format!(".for_{}_start:\n", for_stmt.iterator));
        expression(&for_stmt.range, &mut self.output);

        for stmt in for_stmt.body {
            self.compile_statement(stmt);
        }

        self.output.push_str(&format!(".for_{}_end:\n", for_stmt.iterator));
    }

    pub fn output(&self) -> &str {
        &self.output
    }
}
