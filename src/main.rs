use std::error::Error;
use std::fs::read_to_string;
use std::env;

mod token;
mod lexer;
mod statement;
mod compiler;

use crate::lexer::Lexer;
use crate::token::Token;
use crate::statement::*;
use crate::compiler::Compiler;

#[derive(Debug)]
pub enum AstNode {
    Program(Vec<Statement>),
    FunctionDef(FunctionDef),
    VariableDeclaration(VariableDeclaration),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<&str> = env::args().collect();
    let input;
    
    if args.len() <= 1 {
        return Err("No file was attached!".into());
    }

    input = read_to_string(args[1]).expect("Could not read input!");
    
    // Step 1: Tokenize the input
    let mut lexer = Lexer::new(&input);
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

    Ok(())
}

#[cfg(test)]
mod Tests {
    use super::*;

    #[test]
    fn basic() {

    }
}