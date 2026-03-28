// Parser for C language

use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();
        let mut global_declarations = Vec::new();

        while self.pos < self.tokens.len() {
            match self.peek() {
                Some(Token::EOF) => break,
                Some(Token::Int) | Some(Token::Char) | Some(Token::Void) => {
                    let var_type = self.parse_type()?;
                    let name = self.expect_identifier()?;

                    match self.peek() {
                        Some(Token::OpenParen) => {
                            // Function definition
                            self.advance(); // consume '('
                            let params = self.parse_params()?;
                            self.expect(Token::CloseParen)?;
                            let body = self.parse_statement()?;
                            functions.push(Function {
                                return_type: var_type,
                                name,
                                params,
                                body,
                            });
                        }
                        _ => {
                            // Global variable declaration
                            let init = if let Some(Token::Assign) = self.peek() {
                                self.advance();
                                Some(self.parse_expression()?)
                            } else {
                                None
                            };
                            self.expect(Token::Semicolon)?;
                            global_declarations.push((var_type, name, init));
                        }
                    }
                }
                Some(token) => {
                    return Err(format!("Expected type or function, found {:?}", token));
                }
                None => break,
            }
        }

        Ok(Program {
            functions,
            global_declarations,
        })
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.advance() {
            Some(token) if token == &expected => Ok(()),
            Some(token) => Err(format!("Expected {:?}, found {:?}", expected, token)),
            None => Err(format!("Expected {:?}, found EOF", expected)),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            Some(token) => Err(format!("Expected identifier, found {:?}", token)),
            None => Err("Expected identifier, found EOF".to_string()),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        let mut var_type = match self.advance() {
            Some(Token::Int) => Type::Int,
            Some(Token::Char) => Type::Char,
            Some(Token::Void) => Type::Void,
            Some(token) => return Err(format!("Expected type, found {:?}", token)),
            None => return Err("Expected type, found EOF".to_string()),
        };

        // Handle pointers
        while let Some(Token::Star) = self.peek() {
            self.advance();
            var_type = Type::Pointer(Box::new(var_type));
        }

        Ok(var_type)
    }

    fn parse_params(&mut self) -> Result<Vec<(Type, String)>, String> {
        let mut params = Vec::new();

        if let Some(Token::CloseParen) = self.peek() {
            return Ok(params);
        }

        loop {
            let param_type = self.parse_type()?;
            let param_name = self.expect_identifier()?;
            params.push((param_type, param_name));

            match self.peek() {
                Some(Token::Comma) => {
                    self.advance();
                }
                Some(Token::CloseParen) => break,
                Some(token) => {
                    return Err(format!("Expected ',' or ')', found {:?}", token));
                }
                None => return Err("Expected ',' or ')', found EOF".to_string()),
            }
        }

        Ok(params)
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Some(Token::OpenBrace) => self.parse_block(),
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Do) => self.parse_do_while(),
            Some(Token::For) => self.parse_for(),
            Some(Token::Return) => self.parse_return(),
            Some(Token::Break) => {
                self.advance();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Break)
            }
            Some(Token::Continue) => {
                self.advance();
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Continue)
            }
            Some(Token::Int) | Some(Token::Char) => self.parse_declaration(),
            Some(Token::Semicolon) => {
                self.advance();
                Ok(Stmt::Expr(Expr::IntConst(0)))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Stmt, String> {
        self.expect(Token::OpenBrace)?;
        let mut statements = Vec::new();

        while let Some(token) = self.peek() {
            if token == &Token::CloseBrace {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::CloseBrace)?;
        Ok(Stmt::Block(statements))
    }

    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect(Token::If)?;
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        let then_stmt = Box::new(self.parse_statement()?);

        let else_stmt = if let Some(Token::Else) = self.peek() {
            self.advance();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_stmt,
            else_stmt,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.expect(Token::While)?;
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn parse_do_while(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Do)?;
        let body = Box::new(self.parse_statement()?);
        self.expect(Token::While)?;
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::Semicolon)?;

        Ok(Stmt::DoWhile { body, condition })
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.expect(Token::For)?;
        self.expect(Token::OpenParen)?;

        let init = match self.peek() {
            Some(Token::Semicolon) => {
                self.advance();
                None
            }
            Some(Token::Int) | Some(Token::Char) => {
                let stmt = self.parse_declaration()?;
                Some(Box::new(stmt))
            }
            _ => {
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Some(Box::new(Stmt::Expr(expr)))
            }
        };

        let condition = match self.peek() {
            Some(Token::Semicolon) => {
                self.advance();
                None
            }
            _ => Some(self.parse_expression()?),
        };

        let update = match self.peek() {
            Some(Token::CloseParen) => None,
            _ => Some(Box::new(self.parse_expression()?)),
        };

        self.expect(Token::CloseParen)?;
        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::For {
            init,
            condition,
            update,
            body,
        })
    }

    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Return)?;
        let value = if let Some(Token::Semicolon) = self.peek() {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Return(value))
    }

    fn parse_declaration(&mut self) -> Result<Stmt, String> {
        let var_type = self.parse_type()?;
        let name = self.expect_identifier()?;

        let init = if let Some(Token::Assign) = self.peek() {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(Token::Semicolon)?;
        Ok(Stmt::Declaration {
            var_type,
            name,
            init,
        })
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let left = self.parse_ternary()?;

        if let Some(token) = self.peek() {
            let op = match token {
                Token::Assign => None,
                Token::PlusAssign => Some(BinaryOp::Add),
                Token::MinusAssign => Some(BinaryOp::Sub),
                Token::StarAssign => Some(BinaryOp::Mul),
                Token::SlashAssign => Some(BinaryOp::Div),
                Token::PercentAssign => Some(BinaryOp::Mod),
                Token::AndAssign => Some(BinaryOp::BitAnd),
                Token::OrAssign => Some(BinaryOp::BitOr),
                Token::XorAssign => Some(BinaryOp::BitXor),
                Token::ShiftLeftAssign => Some(BinaryOp::ShiftLeft),
                Token::ShiftRightAssign => Some(BinaryOp::ShiftRight),
                _ => return Ok(left),
            };

            self.advance();

            let right = self.parse_assignment()?;

            if let Some(bin_op) = op {
                return Ok(Expr::Assignment {
                    left: Box::new(left.clone()),
                    right: Box::new(Expr::BinaryOp {
                        op: bin_op,
                        left: Box::new(left),
                        right: Box::new(right),
                    }),
                });
            } else {
                return Ok(Expr::Assignment {
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
        }

        Ok(left)
    }

    fn parse_ternary(&mut self) -> Result<Expr, String> {
        let condition = self.parse_or()?;

        if let Some(Token::Question) = self.peek() {
            self.advance();
            let then_expr = self.parse_expression()?;
            self.expect(Token::Colon)?;
            let else_expr = self.parse_ternary()?;
            Ok(Expr::BinaryOp {
                op: BinaryOp::Equal,
                left: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Equal,
                    left: Box::new(condition),
                    right: Box::new(Expr::IntConst(1)),
                }),
                right: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Equal,
                    left: Box::new(Expr::IntConst(0)),
                    right: Box::new(Expr::BinaryOp {
                        op: BinaryOp::Equal,
                        left: Box::new(then_expr),
                        right: Box::new(else_expr),
                    }),
                }),
            })
        } else {
            Ok(condition)
        }
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;

        while let Some(Token::Or) = self.peek() {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinaryOp {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bitwise_or()?;

        while let Some(Token::And) = self.peek() {
            self.advance();
            let right = self.parse_bitwise_or()?;
            left = Expr::BinaryOp {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bitwise_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bitwise_xor()?;

        while let Some(Token::BitOr) = self.peek() {
            self.advance();
            let right = self.parse_bitwise_xor()?;
            left = Expr::BinaryOp {
                op: BinaryOp::BitOr,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_bitwise_and()?;

        while let Some(Token::BitXor) = self.peek() {
            self.advance();
            let right = self.parse_bitwise_and()?;
            left = Expr::BinaryOp {
                op: BinaryOp::BitXor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;

        while let Some(Token::BitAnd) = self.peek() {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::BinaryOp {
                op: BinaryOp::BitAnd,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_relational()?;

        loop {
            let op = match self.peek() {
                Some(Token::EqualEqual) => Some(BinaryOp::Equal),
                Some(Token::NotEqual) => Some(BinaryOp::NotEqual),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_relational()?;
                left = Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_shift()?;

        loop {
            let op = match self.peek() {
                Some(Token::Less) => Some(BinaryOp::Less),
                Some(Token::LessEqual) => Some(BinaryOp::LessEqual),
                Some(Token::Greater) => Some(BinaryOp::Greater),
                Some(Token::GreaterEqual) => Some(BinaryOp::GreaterEqual),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_shift()?;
                left = Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_shift(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;

        loop {
            let op = match self.peek() {
                Some(Token::ShiftLeft) => Some(BinaryOp::ShiftLeft),
                Some(Token::ShiftRight) => Some(BinaryOp::ShiftRight),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_additive()?;
                left = Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.peek() {
                Some(Token::Plus) => Some(BinaryOp::Add),
                Some(Token::Minus) => Some(BinaryOp::Sub),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_multiplicative()?;
                left = Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match self.peek() {
                Some(Token::Star) => Some(BinaryOp::Mul),
                Some(Token::Slash) => Some(BinaryOp::Div),
                Some(Token::Percent) => Some(BinaryOp::Mod),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_unary()?;
                left = Expr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        let op = match self.peek() {
            Some(Token::Plus) => Some(UnaryOp::Negate),
            Some(Token::Minus) => Some(UnaryOp::Negate),
            Some(Token::Not) => Some(UnaryOp::Not),
            Some(Token::BitNot) => Some(UnaryOp::BitNot),
            Some(Token::PlusPlus) => Some(UnaryOp::PreInc),
            Some(Token::MinusMinus) => Some(UnaryOp::PreDec),
            Some(Token::Star) => Some(UnaryOp::Deref),
            Some(Token::BitAnd) => Some(UnaryOp::Address),
            _ => None,
        };

        if let Some(op) = op {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp { op, operand: Box::new(operand) });
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.peek() {
                Some(Token::OpenParen) => {
                    self.advance();
                    let mut args = Vec::new();

                    if let Some(Token::CloseParen) = self.peek() {
                        self.advance();
                    } else {
                        loop {
                            args.push(self.parse_assignment()?);
                            match self.peek() {
                                Some(Token::Comma) => {
                                    self.advance();
                                }
                                Some(Token::CloseParen) => {
                                    self.advance();
                                    break;
                                }
                                Some(token) => {
                                    return Err(format!("Expected ',' or ')', found {:?}", token));
                                }
                                None => return Err("Expected ',' or ')', found EOF".to_string()),
                            }
                        }
                    }

                    expr = Expr::Call {
                        func: Box::new(expr),
                        args,
                    };
                }
                Some(Token::OpenBracket) => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(Token::CloseBracket)?;
                    expr = Expr::BinaryOp {
                        op: BinaryOp::Add,
                        left: Box::new(expr),
                        right: Box::new(index),
                    };
                    expr = Expr::UnaryOp {
                        op: UnaryOp::Deref,
                        operand: Box::new(expr),
                    };
                }
                Some(Token::Dot) => {
                    self.advance();
                    let field = self.expect_identifier()?;
                    // Simplified: ignore struct field access for now
                    expr = Expr::Identifier(field);
                }
                Some(Token::Arrow) => {
                    self.advance();
                    let field = self.expect_identifier()?;
                    // Simplified: ignore struct field access for now
                    expr = Expr::UnaryOp {
                        op: UnaryOp::Deref,
                        operand: Box::new(Expr::Identifier(field)),
                    };
                }
                Some(Token::PlusPlus) => {
                    self.advance();
                    expr = Expr::UnaryOp {
                        op: UnaryOp::PostInc,
                        operand: Box::new(expr),
                    };
                }
                Some(Token::MinusMinus) => {
                    self.advance();
                    expr = Expr::UnaryOp {
                        op: UnaryOp::PostDec,
                        operand: Box::new(expr),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::IntConst(value)) => Ok(Expr::IntConst(*value)),
            Some(Token::CharConst(value)) => Ok(Expr::CharConst(*value)),
            Some(Token::StringConst(value)) => Ok(Expr::StringConst(value.clone())),
            Some(Token::Identifier(name)) => Ok(Expr::Identifier(name.clone())),
            Some(Token::OpenParen) => {
                let expr = self.parse_expression()?;
                self.expect(Token::CloseParen)?;
                Ok(expr)
            }
            Some(token) => Err(format!("Expected expression, found {:?}", token)),
            None => Err("Expected expression, found EOF".to_string()),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<crate::ast::Program, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
