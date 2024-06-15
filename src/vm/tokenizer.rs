// use regex::Regex;

use crate::error::Error;

#[derive(Debug)]
pub enum SymbolType {
    Instruction,
    Label,
    LabelReference,
    Integer,
    Float,
    String,
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub value: String,
    pub line_number: usize,
    pub column_number: usize,
}

impl Symbol {
    pub fn new(
        symbol_type: SymbolType,
        value: String,
        line_number: usize,
        column_number: usize,
    ) -> Symbol {
        Symbol {
            symbol_type,
            value,
            line_number,
            column_number,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    StringLiteral,
    NumericLiteral,
    Identifier,
    Comment,
    Unknown,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line_number: usize,
    pub column_number: usize,
}

pub struct Tokenizer {
    pub tokens: Vec<Token>,
    line_number: usize,
    column_number: usize,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            tokens: Vec::new(),
            line_number: 1,
            column_number: 1,
        }
    }

    pub fn tokenize(&mut self, code: &str) {
        let mut curr_token = String::new();
        let mut token_type = TokenType::Unknown;

        for c in code.chars() {
            if token_type == TokenType::Comment {
                if c == '\n' {
                    token_type = TokenType::Unknown;
                }
                continue;
            }

            match c {
                '0'..='9' => {
                    if token_type == TokenType::Unknown {
                        token_type = TokenType::NumericLiteral;
                    }

                    curr_token.push(c);
                }
                '.' => match token_type {
                    TokenType::NumericLiteral | TokenType::StringLiteral => {
                        curr_token.push(c);
                    }
                    _ => {
                        // panic!("Invalid token: {}", c);
                        Error::new(
                            "Invalid token",
                            format!("{}:{}", self.line_number, self.column_number),
                        )
                        .print();
                        std::process::exit(1);
                    }
                },
                '"' => {
                    if token_type == TokenType::Unknown {
                        token_type = TokenType::StringLiteral;
                    } else if token_type == TokenType::StringLiteral {
                        self.tokens.push(Token {
                            token_type: TokenType::StringLiteral,
                            value: curr_token.clone(),
                            line_number: self.line_number.clone(),
                            column_number: self.column_number.clone(),
                        });
                        curr_token.clear();
                        token_type = TokenType::Unknown;
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    if token_type == TokenType::Unknown {
                        token_type = TokenType::Identifier;
                    }

                    curr_token.push(c);
                }
                ';' => {
                    token_type = TokenType::Comment;
                }
                ' ' | '\n' | '\t' => {
                    curr_token = curr_token.trim().to_string();

                    if !curr_token.is_empty() {
                        match token_type {
                            TokenType::NumericLiteral => {
                                self.tokens.push(Token {
                                    token_type: TokenType::NumericLiteral,
                                    value: curr_token.clone(),
                                    line_number: self.line_number.clone(),
                                    column_number: self.column_number.clone(),
                                });
                            }
                            TokenType::Identifier => {
                                self.tokens.push(Token {
                                    token_type: TokenType::Identifier,
                                    value: curr_token.clone(),
                                    line_number: self.line_number.clone(),
                                    column_number: self.column_number.clone(),
                                });
                            }
                            TokenType::StringLiteral => {
                                if c == '\n' {
                                    // panic!("Unterminated string literal: {}", curr_token);
                                    Error::new(
                                        "Unterminated string literal",
                                        format!("{}:{}", self.line_number, self.column_number),
                                    )
                                    .print();
                                    std::process::exit(1);
                                }

                                curr_token.push(c);
                                continue;
                            }
                            _ => {}
                        }
                        curr_token.clear();
                        token_type = TokenType::Unknown;
                    }
                }
                _ => {
                    curr_token.push(c);
                }
            }

            if c == '\n' {
                self.line_number += 1;
                self.column_number = 1;
            } else {
                self.column_number += 1;
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            value: String::new(),
            line_number: self.line_number,
            column_number: self.column_number,
        });
    }

    pub fn into_symbols(&self) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        for token in &self.tokens {
            match token.token_type {
                TokenType::NumericLiteral => {
                    if token.value.ends_with('.') {
                        symbols.push(Symbol::new(
                            SymbolType::Float,
                            token.value.clone() + "0",
                            token.line_number,
                            token.column_number,
                        ));
                    } else if token.value.contains('.') {
                        symbols.push(Symbol::new(
                            SymbolType::Float,
                            token.value.clone(),
                            token.line_number,
                            token.column_number,
                        ));
                    } else {
                        symbols.push(Symbol::new(
                            SymbolType::Integer,
                            token.value.clone(),
                            token.line_number,
                            token.column_number,
                        ));
                    }
                }
                TokenType::Identifier => {
                    if token.value.ends_with(':') {
                        symbols.push(Symbol::new(
                            SymbolType::Label,
                            token.value.clone(),
                            token.line_number,
                            token.column_number,
                        ));
                    } else if token.value.starts_with('@') {
                        symbols.push(Symbol::new(
                            SymbolType::LabelReference,
                            token.value.clone(),
                            token.line_number,
                            token.column_number,
                        ));
                    } else {
                        symbols.push(Symbol::new(
                            SymbolType::Instruction,
                            token.value.clone(),
                            token.line_number,
                            token.column_number,
                        ));
                    }
                }
                TokenType::StringLiteral => {
                    symbols.push(Symbol::new(
                        SymbolType::String,
                        token.value.clone(),
                        token.line_number,
                        token.column_number,
                    ));
                }
                _ => {}
            }
        }

        symbols
    }
}
