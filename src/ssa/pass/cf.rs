use std::collections::HashMap;

use crate::ssa::{
    BinaryOperator, ComparisonOperator, Constant, Function, Instruction, InstructionId,
    InstructionKind, Module, Type, Value,
};

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
        let mut foldable_inst = HashMap::new();
        let mut foldable_term = HashMap::new();

        for block_id in &function.block_order {
            let block = function.block(*block_id).unwrap();

            // determine which instructions to fold
            for inst_id in &block.instructions {
                let inst = function.inst(*inst_id).unwrap();
                if let Some(val) = self.fold_inst(inst, &foldable_inst) {
                    foldable_inst.insert(*inst_id, val);
                }
            }

            // determine which terminators to fold
            let inst_id = match block.terminator {
                Some(inst_id) => inst_id,
                None => continue,
            };
            let inst = function.inst(inst_id).unwrap();
            if let Some(kind) = self.fold_term(inst, &foldable_inst) {
                foldable_term.insert(inst_id, kind);
            }
        }

        // fold instructions
        for (inst_id, val) in foldable_inst.into_iter() {
            let inst = function.inst_mut(inst_id).unwrap();
            let users = std::mem::take(&mut inst.users);
            for user_id in users {
                let user_inst = function.inst_mut(user_id).unwrap();
                self.replace_value(user_inst, inst_id, val);
            }
        }

        // fold terminators
        for (inst_id, kind) in foldable_term.into_iter() {
            let inst = function.inst_mut(inst_id).unwrap();
            inst.kind = kind;
        }
    }

    fn replace_value(
        &mut self,
        inst: &mut Instruction,
        find_id: InstructionId,
        replace_value: Value,
    ) {
        for value in inst.values_mut() {
            if let Value::Instruction(inst_val) = value {
                if inst_val.inst_id == find_id {
                    *value = replace_value;
                }
            }
        }
    }

    fn fold_term(
        &mut self,
        inst: &Instruction,
        foldables: &HashMap<InstructionId, Value>,
    ) -> Option<InstructionKind> {
        use InstructionKind::*;

        match inst.kind {
            CondBr(cond, con, alt) => {
                let cond = self.unwrap_i1(&cond, foldables)?;
                match cond {
                    true => Some(InstructionKind::Br(con)),
                    false => Some(InstructionKind::Br(alt)),
                }
            }
            _ => None,
        }
    }

    fn fold_inst(
        &mut self,
        inst: &Instruction,
        foldables: &HashMap<InstructionId, Value>,
    ) -> Option<Value> {
        use crate::ssa::InstructionKind::*;

        match &inst.kind {
            BinOp(op, lhs, rhs) => {
                let lhs = self.unwrap_i32(lhs, foldables)?;
                let rhs = self.unwrap_i32(rhs, foldables)?;
                let val = self.fold_binop(op, lhs, rhs);
                Some(Value::Constant(Constant::I32(val)))
            }
            Cmp(op, lhs, rhs) => match lhs.typ() {
                Type::I1 => {
                    let lhs = self.unwrap_i1(lhs, foldables)?;
                    let rhs = self.unwrap_i1(rhs, foldables)?;
                    let val = self.fold_cmp(op, lhs, rhs);
                    Some(Value::Constant(Constant::I1(val)))
                }
                Type::I32 => {
                    let lhs = self.unwrap_i32(lhs, foldables)?;
                    let rhs = self.unwrap_i32(rhs, foldables)?;
                    let val = self.fold_cmp(op, lhs, rhs);
                    Some(Value::Constant(Constant::I1(val)))
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn fold_binop(&mut self, op: &BinaryOperator, lhs: i32, rhs: i32) -> i32 {
        use BinaryOperator::*;
        match op {
            Add => lhs.wrapping_add(rhs),
            Sub => lhs.wrapping_sub(rhs),
            Mul => lhs.wrapping_mul(rhs),
            Div => lhs.wrapping_div(rhs),
            Rem => lhs.wrapping_rem(rhs),

            Shl => lhs.wrapping_shl(rhs as u32),
            Shr => lhs.wrapping_shr(rhs as u32),

            And => lhs & rhs,
            Or => lhs | rhs,
            Xor => lhs ^ rhs,
        }
    }

    fn fold_cmp<T>(&mut self, op: &ComparisonOperator, lhs: T, rhs: T) -> bool
    where
        T: PartialEq + PartialOrd,
    {
        use ComparisonOperator::*;

        match op {
            Eq => lhs == rhs,
            Neq => lhs != rhs,

            Gt => lhs > rhs,
            Gte => lhs >= rhs,
            Lt => lhs < rhs,
            Lte => lhs <= rhs,
        }
    }

    fn unwrap_i1(
        &mut self,
        val: &Value,
        foldables: &HashMap<InstructionId, Value>,
    ) -> Option<bool> {
        match val {
            Value::Constant(Constant::I1(val)) => Some(*val),
            Value::Instruction(inst_val) => match foldables.get(&inst_val.inst_id)? {
                Value::Constant(Constant::I1(val)) => Some(*val),
                _ => panic!(),
            },
            _ => None,
        }
    }

    fn unwrap_i32(
        &mut self,
        val: &Value,
        foldables: &HashMap<InstructionId, Value>,
    ) -> Option<i32> {
        match val {
            Value::Constant(Constant::I32(val)) => Some(*val),
            Value::Instruction(inst_val) => match foldables.get(&inst_val.inst_id)? {
                Value::Constant(Constant::I32(val)) => Some(*val),
                _ => panic!(),
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantFolding;
    use crate::ssa::{Function, FunctionBuilder, InstructionKind, Type, Value};

    #[test]
    fn cf_1() {
        let mut func_main = Function::new("main", Type::I32, vec![]);
        let mut builder = FunctionBuilder::new(&mut func_main);
        let block_0 = builder.new_block();

        builder.set_block(block_0);
        let v0 = builder.add(Value::new_i32(3), Value::new_i32(4));
        let v1 = builder.mul(v0, Value::new_i32(2));
        let v2 = builder.div(v1, Value::new_i32(1));
        builder.ret(v2);

        // ---

        ConstantFolding::new().apply_function(&mut func_main);

        let ret_id = func_main.block(block_0).unwrap().terminator.unwrap();
        let ret_inst = func_main.inst(ret_id).unwrap();
        let ret_val = ret_inst.values()[0].as_i32();
        assert_eq!(ret_val, 14);
    }

    #[test]
    fn cf_2() {
        let mut func_main = Function::new("main", Type::I32, vec![]);
        let mut builder = FunctionBuilder::new(&mut func_main);
        let block_0 = builder.new_block();

        builder.set_block(block_0);
        let v0 = builder.eq(Value::new_i32(3), Value::new_i32(4));
        builder.ret(v0);

        // ---

        ConstantFolding::new().apply_function(&mut func_main);

        let ret_id = func_main.block(block_0).unwrap().terminator.unwrap();
        let ret_inst = func_main.inst(ret_id).unwrap();
        let ret_val = ret_inst.values()[0].as_i1();
        assert_eq!(ret_val, false);
    }

    #[test]
    fn cf_3() {
        let mut func_main = Function::new("main", Type::I32, vec![]);
        let mut builder = FunctionBuilder::new(&mut func_main);
        let block_0 = builder.new_block();
        let block_1 = builder.new_block();
        let block_2 = builder.new_block();

        builder.set_block(block_0);
        let v0 = builder.eq(Value::new_i32(1), Value::new_i32(1));
        builder.cond_br(v0, block_1, block_2);

        builder.set_block(block_1);
        builder.ret(Value::new_i32(1));

        builder.set_block(block_2);
        builder.ret(Value::new_i32(2));

        // ---

        ConstantFolding::new().apply_function(&mut func_main);

        let br_id = func_main.block(block_0).unwrap().terminator.unwrap();
        let br_inst = func_main.inst(br_id).unwrap();
        // TODO
        assert!(matches!(br_inst.kind, InstructionKind::Br(block_id) if block_id == block_1));
    }
}
