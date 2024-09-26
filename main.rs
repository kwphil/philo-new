mod token;
mod lexer;
mod statement;
mod compiler;

fn main() {
    // Sample input for the Philo language
    let input = r#"
        fn main() returns void {
            let x: %rax = 5;
            let y: %rbx = 10;
            if x < 10 using %rcx {
                x += 2;
            }
        }
        
        fn a() returns u64 {
            let x: %rcx = 1;
            return x;
        }
    "#;

    // Step 1: Tokenize the input
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }
        tokens.push(token);
    }

    // Step 2: Create a simple AST (this part can be extended)
    let ast = AstNode::Program(vec![
        Statement::Function(FunctionDef {
            name: String::from("main"),
            params: vec![],
            return_type: Some(String::from("void")),
            body: vec![
                Statement::VariableDeclaration(VariableDeclaration {
                    name: String::from("x"),
                    var_type: String::from("i32"),
                    value: Box::new(Expression::Number(5)),
                }),
                Statement::If(IfStatement {
                    condition: Box::new(Expression::BinaryOperation {
                        left: Box::new(Expression::Identifier(String::from("x"))),
                        operator: String::from("<"),
                        right: Box::new(Expression::Number(10)),
                    }),
                    then_branch: vec![
                        Statement::VariableDeclaration(VariableDeclaration {
                            name: String::from("x"),
                            var_type: String::from("i32"),
                            value: Box::new(Expression::BinaryOperation {
                                left: Box::new(Expression::Identifier(String::from("x"))),
                                operator: String::from("+"),
                                right: Box::new(Expression::Number(2)),
                            }),
                        }),
                    ],
                    else_branch: None,
                }),
            ],
        }),
    ]);

    // Step 3: Compile the AST to assembly
    let mut compiler = Compiler::new();
    compiler.compile(ast);
    println!("{}", compiler.output());
}
