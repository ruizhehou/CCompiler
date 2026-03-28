// Abstract Syntax Tree definitions for C language

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Void,
    Pointer(Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
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
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
    BitNot,
    Deref,
    Address,
    PostInc,
    PostDec,
    PreInc,
    PreDec,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntConst(i64),
    CharConst(char),
    StringConst(String),
    Identifier(String),
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Assignment {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Cast {
        target_type: Type,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Return(Option<Expr>),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_stmt: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    DoWhile {
        body: Box<Stmt>,
        condition: Expr,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Box<Expr>>,
        body: Box<Stmt>,
    },
    Break,
    Continue,
    Declaration {
        var_type: Type,
        name: String,
        init: Option<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<(Type, String)>,
    pub body: Stmt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
    pub global_declarations: Vec<(Type, String, Option<Expr>)>,
}
