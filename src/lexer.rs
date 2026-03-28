// Lexer for C language

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Int,
    Char,
    Void,
    Return,
    If,
    Else,
    While,
    Do,
    For,
    Break,
    Continue,
    Sizeof,

    // Identifiers and literals
    Identifier(String),
    IntConst(i64),
    CharConst(char),
    StringConst(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Not,
    BitNot,
    ShiftLeft,
    ShiftRight,

    // Assignment
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    ShiftLeftAssign,
    ShiftRightAssign,

    // Punctuation
    Semicolon,
    Comma,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Question,
    Colon,

    // Increment/Decrement
    PlusPlus,
    MinusMinus,

    // Arrow and dot
    Arrow,
    Dot,

    // End of file
    EOF,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();

        match self.peek() {
            None => Ok(Token::EOF),
            Some(c) => match c {
                '0'..='9' => self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                '"' => self.read_string(),
                '\'' => self.read_char(),
                _ => self.read_operator_or_punctuation(),
            },
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.input.next()?;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                None => break,
                Some(c) if c.is_whitespace() => {
                    self.advance();
                }
                Some('/') => {
                    if let Some(&next) = self.input.peek() {
                        if next == '/' {
                            // Single-line comment
                            while let Some(c) = self.advance() {
                                if c == '\n' {
                                    break;
                                }
                            }
                        } else if next == '*' {
                            // Multi-line comment
                            self.advance(); // consume '/'
                            self.advance(); // consume '*'
                            loop {
                                match self.advance() {
                                    Some('*') => {
                                        if let Some(&'/') = self.input.peek() {
                                            self.advance();
                                            break;
                                        }
                                    }
                                    None => {
                                        break; // Unclosed comment
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut value = 0i64;
        let mut has_digits = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                has_digits = true;
                value = value * 10 + (c as i64 - '0' as i64);
                self.advance();
            } else {
                break;
            }
        }

        if !has_digits {
            return Err(format!("Expected number at line {}, column {}", self.line, self.column));
        }

        Ok(Token::IntConst(value))
    }

    fn read_identifier(&mut self) -> Result<Token, String> {
        let mut ident = String::new();

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let token = match ident.as_str() {
            "int" => Token::Int,
            "char" => Token::Char,
            "void" => Token::Void,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "do" => Token::Do,
            "for" => Token::For,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "sizeof" => Token::Sizeof,
            _ => Token::Identifier(ident),
        };

        Ok(token)
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.advance(); // consume opening quote
        let mut s = String::new();

        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    self.advance();
                    return Ok(Token::StringConst(s));
                }
                '\\' => {
                    self.advance();
                    if let Some(escaped) = self.advance() {
                        s.push(match escaped {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '"' => '"',
                            '\'' => '\'',
                            _ => escaped,
                        });
                    }
                }
                _ => {
                    s.push(c);
                    self.advance();
                }
            }
        }

        Err(format!("Unterminated string literal at line {}", self.line))
    }

    fn read_char(&mut self) -> Result<Token, String> {
        self.advance(); // consume opening quote

        let c = match self.advance() {
            Some('\\') => {
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        '0' => '\0',
                        _ => escaped,
                    }
                } else {
                    return Err(format!("Unterminated character literal at line {}", self.line));
                }
            }
            Some(c) => c,
            None => return Err(format!("Unterminated character literal at line {}", self.line)),
        };

        if self.advance() != Some('\'') {
            return Err(format!("Unterminated character literal at line {}", self.line));
        }

        Ok(Token::CharConst(c))
    }

    fn read_operator_or_punctuation(&mut self) -> Result<Token, String> {
        let c = self.advance().unwrap();

        match c {
            '+' => match self.peek() {
                Some('+') => {
                    self.advance();
                    Ok(Token::PlusPlus)
                }
                Some('=') => {
                    self.advance();
                    Ok(Token::PlusAssign)
                }
                _ => Ok(Token::Plus),
            },
            '-' => match self.peek() {
                Some('>') => {
                    self.advance();
                    Ok(Token::Arrow)
                }
                Some('-') => {
                    self.advance();
                    Ok(Token::MinusMinus)
                }
                Some('=') => {
                    self.advance();
                    Ok(Token::MinusAssign)
                }
                _ => Ok(Token::Minus),
            },
            '*' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::StarAssign)
                }
                _ => Ok(Token::Star),
            },
            '/' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::SlashAssign)
                }
                _ => Ok(Token::Slash),
            },
            '%' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::PercentAssign)
                }
                _ => Ok(Token::Percent),
            },
            '=' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::EqualEqual)
                }
                _ => Ok(Token::Assign),
            },
            '!' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::NotEqual)
                }
                _ => Ok(Token::Not),
            },
            '<' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::LessEqual)
                }
                Some('<') => {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        Ok(Token::ShiftLeftAssign)
                    } else {
                        Ok(Token::ShiftLeft)
                    }
                }
                _ => Ok(Token::Less),
            },
            '>' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::GreaterEqual)
                }
                Some('>') => {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        Ok(Token::ShiftRightAssign)
                    } else {
                        Ok(Token::ShiftRight)
                    }
                }
                _ => Ok(Token::Greater),
            },
            '&' => match self.peek() {
                Some('&') => {
                    self.advance();
                    Ok(Token::And)
                }
                Some('=') => {
                    self.advance();
                    Ok(Token::AndAssign)
                }
                _ => Ok(Token::BitAnd),
            },
            '|' => match self.peek() {
                Some('|') => {
                    self.advance();
                    Ok(Token::Or)
                }
                Some('=') => {
                    self.advance();
                    Ok(Token::OrAssign)
                }
                _ => Ok(Token::BitOr),
            },
            '^' => match self.peek() {
                Some('=') => {
                    self.advance();
                    Ok(Token::XorAssign)
                }
                _ => Ok(Token::BitXor),
            },
            '~' => Ok(Token::BitNot),
            ';' => Ok(Token::Semicolon),
            ',' => Ok(Token::Comma),
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '{' => Ok(Token::OpenBrace),
            '}' => Ok(Token::CloseBrace),
            '[' => Ok(Token::OpenBracket),
            ']' => Ok(Token::CloseBracket),
            '?' => Ok(Token::Question),
            ':' => Ok(Token::Colon),
            '.' => Ok(Token::Dot),
            _ => Err(format!(
                "Unexpected character '{}' at line {}, column {}",
                c, self.line, self.column
            )),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(Token::EOF) => break,
            Ok(token) => tokens.push(token),
            Err(e) => return Err(e),
        }
    }

    Ok(tokens)
}
