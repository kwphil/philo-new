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
        let mut returnStr;

        if var_decl.var_type.chars()
                            .next()
                            .unwrap() 
            == '%' 
        {
            returnStr = var_decl.var_type;
        } else {
            returnStr = format!("[{}]", var_decl.name);
        }

        match *var_decl.value {
            Expression::Number(n) => {
                self.output.push_str(&format!("    mov ${}, {}\n", n, &returnStr));
            },

            Expression::Identifier(ref name) => {
                self.output.push_str(&format!("    mov %{}, {}\n", name, &returnStr));
            },

            Expression::BinaryOperation { ref left, ref operator, ref right } => {
                // Compile the left expression
                self.compile_expression(&*left.clone());
                self.output.push_str("    push %rax\n");
                
                // Compile the right expression
                self.compile_expression(&*right.clone());
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
                    _ => unimplemented!("Operator not implemented: {}", operator);
                }

                self.output.push_str(&format!("    mov %rax, %{}\n", var_decl.name));
            },

            _ => unimplemented!("Variable declaration type not implemented"),
        }
    }

    
}