use crate::utils::Error;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Lambda,

    Eof,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Num(f64),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub fn eof(line: u32) -> Token {
        Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: None,
            line,
        }
    }
}

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    pub errors: Vec<Error>,
    current_line: u32,
    lexeme_start_idx: usize,
    current_idx: usize,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            errors: Vec::new(),
            current_line: 1,
            lexeme_start_idx: 0,
            current_idx: 0,
        }
    }

    pub fn scan_tokens(&mut self, source: String) {
        self.source = source.chars().collect();

        while !self.exhausted_chars() {
            self.lexeme_start_idx = self.current_idx;
            self.scan_single_token();
        }

        self.tokens.push(Token::eof(self.current_line));
    }

    fn scan_single_token(&mut self) {
        let token = self.advance();

        // normal tokens
        if let Some(token_type) = match token {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBracket),
            '}' => Some(TokenType::RightBracket),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '!' => {
                if self.match_advance('=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_advance('=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_advance('=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            }
            '>' => {
                if self.match_advance('=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_advance('/') {
                    // Commented line
                    while self.peek() != '\n' && !self.exhausted_chars() {
                        self.advance();
                    }
                    None
                } else {
                    Some(TokenType::Slash)
                }
            }
            _ => None,
        } {
            self.tokens.push(Token {
                token_type,
                lexeme: self.source[self.lexeme_start_idx..self.current_idx]
                    .iter()
                    .cloned()
                    .collect(),
                literal: None,
                line: self.current_line,
            });
        } else {
            // whitespace tokens
            match token {
                ' ' => (),
                '\r' => (),
                '\t' => (),
                '\n' => self.current_line += 1,
                // treat comments like whitespace
                '/' => (),
                _ => self.errors.push(Error {
                    message: String::from("Unexpected character"),
                    line: self.current_line,
                }),
            }
        }
    }

    fn peek(&mut self) -> char {
        if self.exhausted_chars() {
            '\0'
        } else {
            self.source[self.current_idx]
        }
    }

    fn advance(&mut self) -> char {
        if self.exhausted_chars() {
            '\0'
        } else {
            self.current_idx += 1;
            self.source[self.current_idx - 1]
        }
    }

    fn match_advance(&mut self, expected_next: char) -> bool {
        if self.exhausted_chars() || self.source[self.current_idx] != expected_next {
            false
        } else {
            self.current_idx += 1;
            true
        }
    }

    fn exhausted_chars(&self) -> bool {
        self.current_idx >= self.source.len()
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn report_errors(&self) {
        for error in self.errors.iter() {
            error.report();
        }
    }
}

pub fn get_tokens(source: String) -> Result<Vec<Token>, Vec<Error>> {
    let mut scanner = Scanner::new();

    scanner.scan_tokens(source);

    if scanner.has_errors() {
        scanner.report_errors();
        Err(scanner.errors)
    } else {
        Ok(scanner.tokens)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_scan_1() {
        let source = String::from("(");
        let tokens = get_tokens(source).unwrap();
        let expected = vec![
            Token {
                token_type: TokenType::LeftParen,
                lexeme: String::from("("),
                literal: None,
                line: 1,
            },
            Token::eof(1),
        ];
        assert_eq!(&tokens[..], &expected[..]);
    }

    #[test]
    fn test_scan_2() {
        let source = String::from("()");
        let tokens = get_tokens(source).unwrap();
        let expected = vec![
            Token {
                token_type: TokenType::LeftParen,
                lexeme: String::from("("),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                lexeme: String::from(")"),
                literal: None,
                line: 1,
            },
            Token::eof(1),
        ];
        assert_eq!(&tokens[..], &expected[..]);
    }

    #[test]
    fn test_scan_3() {
        let source = String::from(
            "!=
// aaa
>= /",
        );
        let tokens = get_tokens(source).unwrap();
        let expected = vec![
            Token {
                token_type: TokenType::BangEqual,
                lexeme: String::from("!="),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::GreaterEqual,
                lexeme: String::from(">="),
                literal: None,
                line: 3,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: String::from("/"),
                literal: None,
                line: 3,
            },
            Token::eof(3),
        ];
        assert_eq!(&tokens[..], &expected[..]);
    }

    #[test]
    fn test_scan_4() {
        let source = String::from(
            "// this is a comment
(( )){} // grouping stuff
>= / // operators",
        );
        let tokens = get_tokens(source).unwrap();
        let expected = vec![
            Token {
                token_type: TokenType::LeftParen,
                lexeme: String::from("("),
                literal: None,
                line: 2,
            },
            Token {
                token_type: TokenType::LeftParen,
                lexeme: String::from("("),
                literal: None,
                line: 2,
            },
            Token {
                token_type: TokenType::RightParen,
                lexeme: String::from(")"),
                literal: None,
                line: 2,
            },
            Token {
                token_type: TokenType::RightParen,
                lexeme: String::from(")"),
                literal: None,
                line: 2,
            },
            Token {
                token_type: TokenType::LeftBracket,
                lexeme: String::from("{"),
                literal: None,
                line: 2,
            },
            Token {
                token_type: TokenType::RightBracket,
                lexeme: String::from("}"),
                literal: None,
                line: 2,
            },
            Token {
                token_type: TokenType::GreaterEqual,
                lexeme: String::from(">="),
                literal: None,
                line: 3,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: String::from("/"),
                literal: None,
                line: 3,
            },
            Token::eof(3),
        ];
        assert_eq!(&tokens[..], &expected[..]);
    }
}
