use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer {
            input,
            position: 0,
            current_char: input.chars().next(),
        };
        
        lexer
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn identifier(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].to_string()
    }

    fn number(&mut self) -> i64 {
        let start = self.position;

        while let Some(ch) = self.current_char {
            if ch.is_digit(10) {
                self.advance();
            } else {
                break;
            }

            self.input[start..self.position].parse().unwrap()
        }
    }

    fn string_literal(&mut self) -> String {
        self.advance();
        let start = self.position;

        while let Some(ch) = self.current_char {
            if ch == '"' {
                break;
            }

            self.advance();
        }

        let literal = self.input[start..self.position].to_string();
        self.advance();
        
        literal
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\n' | '\r' | '\t' => {
                    self.skip_whitespace();
                    return Token::Whitespace;
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.identifier();
                    return match ident.as_str() {
                        "fn" | "let" | "if" | "else" | "while" | "for" | "struct" | "impl" | "enum" | "pub" => {
                            Token::Keyword(ident)
                        },

                        _ => Token::Identifier(ident),
                    };
                }

                '0'..='9' => {
                    return Token::Number(self.number());
                }

                '"' => {
                    return Token::StringLiteral(self.string_literal());
                }

                ';' | '{' | '}' | '(' | ')' | ',' => {
                    let symbol = ch;
                    self.advance();
                    
                    return Token::Symbol(symbol);
                }

                '/' => {
                    self.advance();

                    // for division
                    if self.current_char != Some('/') {
                        return Token::Operator('/'.to_string());
                    }
                    
                    // for comment
                    while self.current_char != Some('\n') {
                        self.advance();
                    }

                    return Token::Comment;
                }

                '+' | '-' | '*' | '=' | '<' | '>' => {
                    let op = ch.to_string();
                    self.advance();

                    return Token::Operator(op);
                }

                _ => {
                    self.advance();
                }
            }
        }

        Token::Eof
    }
}