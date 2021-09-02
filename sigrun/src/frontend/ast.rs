mod stmt;
pub use stmt::*;

mod expr;
pub use expr::*;

use crate::common::{pos::Pos, symtab::NodeId, types::Type};

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
    pub global_vars: Vec<GlobalVar>,
    pub id: NodeId,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            global_vars: Vec::new(),
            id: NodeId::new(),
        }
    }
}

#[derive(Debug)]
pub struct GlobalVar {
    pub name: String,
    pub typ: Type,
    pub is_const: bool,
    pub id: NodeId,
}

impl From<StatementKind> for GlobalVar {
    fn from(kind: StatementKind) -> Self {
        match kind {
            StatementKind::Var { name, typ, .. } => GlobalVar {
                name,
                typ,
                is_const: false,
                id: NodeId::new(),
            },
            StatementKind::Val { name, typ, .. } => GlobalVar {
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
