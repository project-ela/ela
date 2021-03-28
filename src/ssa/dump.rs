use super::{
    BinaryOperator, BlockId, ComparisonOperator, Constant, Function, Global, GlobalValue,
    InstructionId, InstructionValue, Module, ParameterValue, Terminator, Type, Types, Value,
};

impl Module {
    pub fn dump(&self) -> String {
        let global_str = self
            .globals
            .iter()
            .map(|(_, global)| global.dump(self))
            .collect::<Vec<String>>()
            .join("\n");

        let function_str = self
            .functions
            .iter()
            .map(|(_, function)| function.dump(self))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n\n{}", global_str, function_str)
    }
}

impl Global {
    fn dump(&self, module: &Module) -> String {
        format!("@{}: {}", self.name, self.typ.dump(&module.types))
    }
}

impl Function {
    pub fn dump(&self, module: &Module) -> String {
        let param_str = self
            .param_typ
            .iter()
            .enumerate()
            .map(|(i, typ)| format!("{} %{}", typ.dump(&module.types), i))
            .collect::<Vec<String>>()
            .join(", ");

        let block_str = self
            .blocks
            .iter()
            .map(|(block_id, _)| self.dump_block(module, block_id))
            .collect::<Vec<String>>()
            .join("\n\n");

        format!(
            "func {}({}) {} {{\n{}\n}}\n",
            self.name,
            param_str,
            self.ret_typ.dump(&module.types),
            block_str
        )
    }

    fn dump_block(&self, module: &Module, block_id: BlockId) -> String {
        let block = self.blocks.get(block_id).unwrap();

        let inst_str = block
            .instructions
            .iter()
            .map(|inst_id| self.dump_inst(module, *inst_id))
            .collect::<Vec<String>>()
            .join("\n");

        let term_str = match block.terminator {
            Some(term_id) => self.term(term_id).unwrap().dump(module, self),
            None => "  invalid".into(),
        };

        match inst_str.is_empty() {
            false => format!("  b{}:\n{}\n{}", block_id.index(), inst_str, term_str),
            true => format!("  b{}:\n{}", block_id.index(), term_str),
        }
    }

    fn dump_inst(&self, module: &Module, inst_id: InstructionId) -> String {
        use super::Instruction::*;

        let inst = self.instructions.get(inst_id).unwrap();
        let inst_str = match inst {
            BinOp(op, lhs, rhs) => format!(
                "{} {}, {}",
                op.dump(),
                lhs.dump(module, self),
                rhs.dump(module, self)
            ),
            Cmp(op, lhs, rhs) => format!(
                "{} {}, {}",
                op.dump(),
                lhs.dump(module, self),
                rhs.dump(module, self)
            ),

            Call(func_id, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.dump(module, self))
                    .collect::<Vec<String>>()
                    .join(", ");

                let func_name = &module.function(*func_id).unwrap().name;
                match args.len() {
                    0 => format!("call {}", func_name),
                    _ => format!("call {}, {}", func_name, args_str),
                }
            }
            Arg(index) => format!("arg {}", index),

            Alloc(typ) => format!("alloc {}", typ.dump(&self.types)),
            Load(src) => format!("load {}", src.dump(module, self)),
            Store(dst, src) => format!(
                "store {}, {}",
                dst.dump(module, self),
                src.dump(module, self)
            ),
        };

        format!("    %{} = {}", inst_id.index(), inst_str)
    }
}

impl Terminator {
    fn dump(&self, module: &Module, func: &Function) -> String {
        use super::Terminator::*;

        match self {
            Ret(Some(val)) => format!("  ret {}", val.dump(module, func)),
            Ret(None) => format!("  ret"),
            Br(dst) => format!("  br b{}", dst.index()),
            CondBr(cond, con, alt) => {
                format!(
                    "  br {} -> b{} b{} ",
                    cond.dump(module, func),
                    con.index(),
                    alt.index()
                )
            }
        }
    }
}

impl BinaryOperator {
    fn dump(&self) -> String {
        use super::BinaryOperator::*;

        match self {
            Add => "add",
            Sub => "sub",
            Mul => "mul",
            Div => "div",
            Rem => "rem",
            Shl => "shl",
            Shr => "shr",
            And => "and",
            Or => "or",
            Xor => "xor",
        }
        .into()
    }
}

impl ComparisonOperator {
    fn dump(&self) -> String {
        use super::ComparisonOperator::*;

        match self {
            Eq => "eq",
            Neq => "neq",
            Gt => "gt",
            Gte => "gte",
            Lt => "lt",
            Lte => "lte",
        }
        .into()
    }
}

impl Value {
    fn dump(&self, module: &Module, func: &Function) -> String {
        use super::Value::*;

        let typ_str = match self {
            Global(_) => self.typ().dump(&module.types),
            _ => self.typ().dump(&func.types),
        };

        match self {
            Constant(r#const) => format!("{} {}", typ_str, r#const.dump()),
            Instruction(InstructionValue { inst_id, typ: _ }) => {
                format!("{} %{}", typ_str, inst_id.index())
            }
            Parameter(ParameterValue { index, typ: _ }) => {
                format!("{} %{}", typ_str, index)
            }
            Global(GlobalValue { global_id, typ: _ }) => {
                let name = &module.global(*global_id).unwrap().name;
                format!("{} @{}", typ_str, name)
            }
        }
    }
}

impl Type {
    fn dump(&self, types: &Types) -> String {
        use super::Type::*;

        match self {
            Void => "void".into(),
            I1 => "i1".into(),
            I32 => "i32".into(),
            Pointer(_) => {
                let elm_typ = types.elm_typ(*self);
                format!("*{}", elm_typ.dump(types))
            }
        }
    }
}

impl Constant {
    fn dump(&self) -> String {
        use super::Constant::*;

        match self {
            I1(x) => format!("{}", *x as u32),
            I32(x) => format!("{}", x),
        }
    }
}
