use crate::token::ASSIGN;
use crate::token::COMMA;
use crate::token::EOF;
use crate::token::IDENT;
use crate::token::ILLEGAL;
use crate::token::LBRACE;
use crate::token::LPAREN;
use crate::token::PLUS;
use crate::token::RBRACE;
use crate::token::RPAREN;
use crate::token::SEMICOLON;
use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: u8,
}

fn is_letter(ch: u8) -> bool {
    (b'a'..=b'z').contains(&ch) || (b'A'..=b'Z').contains(&ch)
}

fn read_identifier(l: &mut Lexer) -> String {
    let position = l.position;

    while is_letter(l.ch) {
        l.read_char()
    }
    let sub = &l.input[position as usize..l.position as usize];
    return sub.to_string();
}

impl Lexer {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len().try_into().unwrap() {
            self.ch = 0;
        } else {
            println!("current position {}", self.read_position);
            self.ch = self
                .input
                .chars()
                .nth(self.read_position.try_into().unwrap())
                .unwrap() as u8;

            println!("{}, what", self.ch);
        }
        self.position = self.read_position;
        self.read_position += 1;
        println!("{:?}", self);
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            b'=' => Token {
                token_type: ASSIGN.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b';' => Token {
                token_type: SEMICOLON.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b'(' => Token {
                token_type: LPAREN.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b')' => Token {
                token_type: RPAREN.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b'{' => Token {
                token_type: LBRACE.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b'}' => Token {
                token_type: RBRACE.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b'+' => Token {
                token_type: PLUS.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b',' => Token {
                token_type: COMMA.to_string(),
                literal: (self.ch as char).to_string(),
            },
            b'\0' => Token {
                token_type: EOF.to_string(),
                literal: ("").to_string(),
            },
            _ => {
                if is_letter(self.ch) {
                    Token {
                        token_type: IDENT.to_string(),
                        literal: read_identifier(self),
                    }
                } else {
                    Token {
                        token_type: ILLEGAL.to_string(),
                        literal: (self.ch as char).to_string(),
                    }
                }
            }
        };

        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token;

    #[test]
    fn test_simple_tokens() {
        let sample_input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);";

        struct ExpectedToken {
            expected_token_type: token::TokenType,
            expected_literal: String,
        }
        let tests: [ExpectedToken; 27] = [
            ExpectedToken {
                expected_token_type: token::LET.to_string(),
                expected_literal: String::from("let"),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("five"),
            },
            ExpectedToken {
                expected_token_type: token::ASSIGN.to_string(),
                expected_literal: String::from("="),
            },
            ExpectedToken {
                expected_token_type: token::INT.to_string(),
                expected_literal: String::from("5"),
            },
            ExpectedToken {
                expected_token_type: token::SEMICOLON.to_string(),
                expected_literal: String::from(";"),
            },
            ExpectedToken {
                expected_token_type: token::LET.to_string(),
                expected_literal: String::from("let"),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("ten"),
            },
            ExpectedToken {
                expected_token_type: token::ASSIGN.to_string(),
                expected_literal: String::from("="),
            },
            ExpectedToken {
                expected_token_type: token::INT.to_string(),
                expected_literal: String::from("10"),
            },
            ExpectedToken {
                expected_token_type: token::SEMICOLON.to_string(),
                expected_literal: String::from(";"),
            },
            ExpectedToken {
                expected_token_type: token::LET.to_string(),
                expected_literal: String::from("let"),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("add"),
            },
            ExpectedToken {
                expected_token_type: token::FUNCTION.to_string(),
                expected_literal: String::from("fn"),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("x"),
            },
            ExpectedToken {
                expected_token_type: token::COMMA.to_string(),
                expected_literal: String::from(","),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("y"),
            },
            ExpectedToken {
                expected_token_type: token::RBRACE.to_string(),
                expected_literal: String::from("}"),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("result"),
            },
            ExpectedToken {
                expected_token_type: token::ASSIGN.to_string(),
                expected_literal: String::from("="),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("add"),
            },
            ExpectedToken {
                expected_token_type: token::LPAREN.to_string(),
                expected_literal: String::from("("),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("five"),
            },
            ExpectedToken {
                expected_token_type: token::COMMA.to_string(),
                expected_literal: String::from(","),
            },
            ExpectedToken {
                expected_token_type: token::IDENT.to_string(),
                expected_literal: String::from("ten"),
            },
            ExpectedToken {
                expected_token_type: token::RPAREN.to_string(),
                expected_literal: String::from(")"),
            },
            ExpectedToken {
                expected_token_type: token::SEMICOLON.to_string(),
                expected_literal: String::from(";"),
            },
            ExpectedToken {
                expected_token_type: token::EOF.to_string(),
                expected_literal: String::from("EOF"),
            },
        ];

        let mut l = Lexer {
            input: sample_input.to_string(),
            ch: sample_input.chars().nth(0).unwrap() as u8,
            position: 0,
            read_position: 1,
        };

        for (_i, v) in tests.iter().enumerate() {
            let tok = l.next_token();
            println!(" token in test {:?}", tok);

            assert_eq!(
                v.expected_token_type, tok.token_type,
                "Expected token type {}, but got token type{}",
                v.expected_token_type, tok.token_type,
            );

            assert_eq!(
                v.expected_literal, tok.literal,
                "Expected literal {}, but got literal {}",
                v.expected_literal, tok.literal,
            );
        }
    }
}
