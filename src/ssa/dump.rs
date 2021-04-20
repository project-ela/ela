use super::{
    BinaryOperator, BlockId, ComparisonOperator, Constant, Function, Global, GlobalValue,
    InstructionId, InstructionValue, Module, ParameterValue, Type, Types, Value,
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
        format!(
            "@{} = {} {}",
            self.name,
            self.typ.dump(&module.types.borrow()),
            self.init_value.dump()
        )
    }
}

impl Function {
    pub fn dump(&self, module: &Module) -> String {
        let param_str = self
            .param_typ
            .iter()
            .enumerate()
            .map(|(i, typ)| format!("{} %{}", typ.dump(&self.types.borrow()), i))
            .collect::<Vec<String>>()
            .join(", ");

        let block_str = self
            .block_order
            .iter()
            .map(|block_id| self.dump_block(module, *block_id))
            .collect::<Vec<String>>()
            .join("\n\n");

        format!(
            "func @{}({}) {} {{\n{}\n}}\n",
            self.name,
            param_str,
            self.ret_typ.dump(&self.types.borrow()),
            block_str
        )
    }

    fn dump_block(&self, module: &Module, block_id: BlockId) -> String {
        let block = self.block(block_id).unwrap();

        let inst_str = block
            .instructions
            .iter()
            .map(|inst_id| self.dump_inst(module, *inst_id))
            .collect::<Vec<String>>()
            .join("\n");

        let term_str = match block.terminator {
            Some(inst_id) => self.dump_inst(module, inst_id),
            None => "".into(),
        };

        match inst_str.is_empty() {
            false => format!("  b{}:\n{}\n{}", block_id.index(), inst_str, term_str),
            true => format!("  b{}:\n{}", block_id.index(), term_str),
        }
    }

    fn dump_inst(&self, module: &Module, inst_id: InstructionId) -> String {
        use super::InstructionKind::*;

        let inst = self.instructions.get(inst_id).unwrap();
        let inst_str = match &inst.kind {
            BinOp(op, lhs, rhs) => {
                format!("{} {}, {}", op.dump(), lhs.dump(module), rhs.dump(module))
            }
            Cmp(op, lhs, rhs) => {
                format!("{} {}, {}", op.dump(), lhs.dump(module), rhs.dump(module))
            }

            Call(func_id, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.dump(module))
                    .collect::<Vec<String>>()
                    .join(", ");

                let func_name = &module.function(*func_id).unwrap().name;
                match args.len() {
                    0 => format!("call @{}", func_name),
                    _ => format!("call @{}({})", func_name, args_str),
                }
            }
            Param(index) => format!("param {}", index),

            Alloc(typ) => format!("alloc {}", typ.dump(&self.types.borrow())),
            Load(src) => format!("load {}", src.dump(module)),
            Store(dst, src) => format!("store {}, {}", dst.dump(module), src.dump(module)),

            Gep(val, indices) => {
                let indices_str = indices
                    .iter()
                    .map(|index| format!("{}", index.dump(module)))
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("gep {}, {}", val.dump(module), indices_str)
            }

            Ret(Some(val)) => format!("  ret {}", val.dump(module)),
            Ret(None) => format!("  ret"),
            Br(dst) => format!("  br b{}", dst.index()),
            CondBr(cond, con, alt) => {
                format!(
                    "  br {} -> b{} b{} ",
                    cond.dump(module),
                    con.index(),
                    alt.index()
                )
            }
        };
        let users_str = inst
            .users
            .iter()
            .map(|user| format!("%{}", user.index()))
            .collect::<Vec<String>>()
            .join(", ");

        match inst.is_terminator() {
            true => format!("  {}", inst_str),
            false => format!("    %{} = {} <-- {}", inst_id.index(), inst_str, users_str),
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
    fn dump(&self, module: &Module) -> String {
        use super::Value::*;

        let typ_str = self.typ().dump(&module.types.borrow());

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
            Array(_, len) => {
                let elm_typ = types.elm_typ(*self);
                format!("[{}]{}", len, elm_typ.dump(types))
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
