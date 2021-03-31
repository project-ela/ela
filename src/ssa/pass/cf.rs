use std::collections::HashMap;

use crate::ssa::{BinaryOperator, Constant, Function, Instruction, InstructionId, Module, Value};

pub fn apply(module: &mut Module) {
    ConstantFolding::new().apply(module);
}

struct ConstantFolding {}

impl ConstantFolding {
    fn new() -> Self {
        Self {}
    }

    fn apply(&mut self, module: &mut Module) {
        for (_, function) in module.functions.iter_mut() {
            self.apply_function(function);
        }
    }

    fn apply_function(&mut self, function: &mut Function) {
        let mut foldables = HashMap::new();

        for (_, block) in &function.blocks {
            for inst_id in &block.instructions {
                let inst = function.inst(*inst_id).unwrap();
                if let Some(val) = self.fold(inst, &foldables) {
                    foldables.insert(*inst_id, val);
                }
            }
        }

        for (inst_id, val) in foldables.into_iter() {
            let inst = function.inst_mut(inst_id).unwrap();
            let users = std::mem::take(&mut inst.users_inst);
            for user_id in users {
                let user_inst = function.inst_mut(user_id).unwrap();
                for value in user_inst.kind.values_mut() {
                    if let Value::Instruction(inst_val) = value {
                        if inst_val.inst_id == inst_id {
                            let _ = std::mem::replace(value, val);
                        }
                    }
                }
            }

            let inst = function.inst_mut(inst_id).unwrap();
            let users = std::mem::take(&mut inst.users_term);
            for user_id in users {
                let user_term = function.term_mut(user_id).unwrap();
                for value in user_term.values_mut() {
                    if let Value::Instruction(inst_val) = value {
                        if inst_val.inst_id == inst_id {
                            let _ = std::mem::replace(value, val);
                        }
                    }
                }
            }
        }
    }

    fn fold(
        &mut self,
        inst: &Instruction,
        foldables: &HashMap<InstructionId, Value>,
    ) -> Option<Value> {
        use crate::ssa::InstructionKind::*;

        match &inst.kind {
            BinOp(op, lhs, rhs) => {
                let lhs = self.unwrap_const(lhs, foldables)?;
                let rhs = self.unwrap_const(rhs, foldables)?;
                let val = self.fold_binop(op, lhs, rhs);
                Some(Value::Constant(Constant::I32(val)))
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn fold_binop(&mut self, op: &BinaryOperator, lhs: i32, rhs: i32) -> i32 {
        use BinaryOperator::*;
        match op {
            Add => lhs.wrapping_add(rhs),
            x => unimplemented!("{:?}", x),
        }
    }

    fn unwrap_const(
        &mut self,
        val: &Value,
        foldables: &HashMap<InstructionId, Value>,
    ) -> Option<i32> {
        match val {
            Value::Constant(r#const) => match r#const {
                Constant::I32(val) => Some(*val),
                _ => None,
            },
            Value::Instruction(inst_val) => Some(foldables.get(&inst_val.inst_id)?.as_i32()),
            _ => None,
        }
    }
}
