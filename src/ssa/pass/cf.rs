use std::collections::HashMap;

use crate::ssa::{
    BinaryOperator, ComparisonOperator, Constant, Function, Instruction, InstructionId, Module,
    Type, Value,
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
            let users = std::mem::take(&mut inst.users);
            for user_id in users {
                let user_inst = function.inst_mut(user_id).unwrap();
                self.replace_value(user_inst, inst_id, val);
            }
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

    fn fold(
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
    use crate::ssa::{Function, FunctionBuilder, Type, Value};

    #[test]
    fn cf_1() {
        let mut func_main = Function::new("main", Type::I32, vec![]);
        let mut builder = FunctionBuilder::new(&mut func_main);
        let block_0 = builder.add_block();

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
        let block_0 = builder.add_block();

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
}
