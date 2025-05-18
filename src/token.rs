pub static ILLEGAL: &str = "ILLEGAL";
pub static EOF: &str = "EOF";
pub static IDENT: &str = "IDENT";
pub static INT: &str = "INT";
pub static LPAREN: &str = "(";
pub static RPAREN: &str = ")";
pub static LBRACE: &str = "{";
pub static RBRACE: &str = "}";
pub static PLUS: &str = "+";
pub static COMMA: &str = ",";
pub static SEMICOLON: &str = ";";
pub static FUNCTION: &str = "FUNCTION";
pub static LET: &str = "LET";
pub static ASSIGN: &str = "=";

pub type TokenType = String;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: String::new(),
            literal: String::new(),
        }
    }
}
