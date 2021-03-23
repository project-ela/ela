use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    pos::Pos,
    symtab::NodeId,
    types::Type,
};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub global_defs: Vec<GlobalDef>,
    pub id: NodeId,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            global_defs: Vec::new(),
            id: NodeId::new(),
        }
    }
}

#[derive(Debug)]
pub struct GlobalDef {
    pub name: String,
    pub typ: Type,
    pub is_const: bool,
    pub id: NodeId,
}

impl From<StatementKind> for GlobalDef {
    fn from(kind: StatementKind) -> Self {
        match kind {
            StatementKind::Var { name, typ, .. } => GlobalDef {
                name,
                typ,
                is_const: false,
                id: NodeId::new(),
            },
            StatementKind::Val { name, typ, .. } => GlobalDef {
                name,
                typ,
                is_const: true,
                id: NodeId::new(),
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub ret_typ: Type,
    pub body: Option<Statement>,
    pub pos: Pos,
    pub id: NodeId,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub pos: Pos,
    pub id: NodeId,
}

impl Statement {
    pub fn new(kind: StatementKind, pos: Pos) -> Self {
        Self {
            kind,
            pos,
            id: NodeId::new(),
        }
    }
}

#[derive(Debug)]
pub enum StatementKind {
    Block {
        stmts: Vec<Statement>,
    },

    Var {
        name: String,
        typ: Type,
        value: Option<Box<Expression>>,
    },
    Val {
        name: String,
        typ: Type,
        value: Option<Box<Expression>>,
    },
    Assign {
        dst: Box<Expression>,
        value: Box<Expression>,
    },
    Return {
        value: Option<Box<Expression>>,
    },
    If {
        cond: Box<Expression>,
        then: Box<Statement>,
        els: Option<Box<Statement>>,
    },
    While {
        cond: Box<Expression>,
        body: Box<Statement>,
    },
    // use this if return type is void
    Call {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub pos: Pos,
    pub id: NodeId,
}

impl Expression {
    pub fn new(kind: ExpressionKind, pos: Pos) -> Self {
        Self {
            kind,
            pos,
            id: NodeId::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Char {
        value: char,
    },
    Integer {
        value: i32,
    },
    String {
        value: String,
    },
    Bool {
        value: bool,
    },
    Ident {
        name: String,
    },

    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    BinaryOp {
        op: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    // use this if return type isn't void
    Call {
        name: String,
        args: Vec<Expression>,
    },
    Index {
        lhs: Box<Expression>,
        index: Box<Expression>,
    },
}
