use anyhow::Result;

use crate::{
    common::{
        error::{Error, Errors},
        pos::Pos,
        types::Type,
    },
    frontend::{ast::Module, pass::error::PassError},
};

pub fn apply(module: &Module) -> Result<()> {
    let mut pass = SemaCheck::new();
    pass.apply(module);
    match pass.issues.0.len() {
        0 => Ok(()),
        _ => Err(pass.issues.into()),
    }
}

#[derive(Debug)]
struct SemaCheck {
    issues: Errors,
}

impl SemaCheck {
    fn new() -> Self {
        Self {
            issues: Errors::default(),
        }
    }

    fn apply(&mut self, module: &Module) {
        let mut main_exists = false;
        for function in &module.functions {
            if function.name != "main" {
                continue;
            }
            main_exists = true;

            if function.ret_typ != Type::Int {
                self.issue(function.pos.clone(), PassError::MainShouldReturnInt);
            }
        }

        if !main_exists {
            self.issue(Pos::default(), PassError::MainNotFound);
        }
    }

    fn issue(&mut self, pos: Pos, err: PassError) {
        self.issues.0.push(Error::new(pos, err));
    }
}
