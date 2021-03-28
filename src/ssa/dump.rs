use super::{BlockId, Constant, Function, InstructionId, Module, Type, Value};

impl Module {
    pub fn dump(&self) -> String {
        self.functions
            .iter()
            .map(|(_, function)| function.dump(self))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Function {
    pub fn dump(&self, module: &Module) -> String {
        let param_str = self
            .param_typ
            .iter()
            .enumerate()
            .map(|(i, typ)| format!("{} %{}", typ.dump(self), i))
            .collect::<Vec<String>>()
            .join(", ");

        let block_str = self
            .blocks
            .iter()
            .map(|(block_id, _)| self.dump_block(module, block_id))
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "func {}({}) {} {{\n{}\n}}\n",
            self.name,
            param_str,
            self.ret_typ.dump(self),
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

        format!("  b{}:\n{}", block_id.index(), inst_str)
    }

    fn dump_inst(&self, module: &Module, inst_id: InstructionId) -> String {
        use super::Instruction::*;

        let inst = self.instructions.get(inst_id).unwrap();
        let inst_str = match inst {
            Add(lhs, rhs) => format!("add {}, {}", lhs.dump(self), rhs.dump(self)),
            Equal(lhs, rhs) => format!("eq {}, {}", lhs.dump(self), rhs.dump(self)),

            Call(func_id, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.dump(self))
                    .collect::<Vec<String>>()
                    .join(", ");

                let func_name = &module.function(*func_id).unwrap().name;
                match args.len() {
                    0 => format!("call {}", func_name),
                    _ => format!("call {}, {}", func_name, args_str),
                }
            }
            Arg(index) => format!("arg {}", index),

            Alloc(typ) => format!("alloc {}", typ.dump(self)),
            Load(src) => format!("load {}", src.dump(self)),
            Store(dst, src) => format!("store {}, {}", dst.dump(self), src.dump(self)),

            Ret(val) => format!("ret {}", val.dump(self)),
            Br(dst) => format!("br b{}", dst.index()),
            CondBr(cond, con, alt) => {
                format!(
                    "br {} -> b{} b{} ",
                    cond.dump(self),
                    con.index(),
                    alt.index()
                )
            }
        };

        format!("    %{} = {}", inst_id.index(), inst_str)
    }
}

impl Value {
    fn dump(&self, func: &Function) -> String {
        use super::Value::*;

        match self {
            Constant(r#const) => format!("{} {}", self.typ().dump(func), r#const.dump()),
            Instruction(inst_val) => {
                format!("{} %{}", self.typ().dump(func), inst_val.inst_id.index())
            }
            Parameter(param_val) => format!("{} %{}", self.typ().dump(func), param_val.index),
        }
    }
}

impl Type {
    fn dump(&self, func: &Function) -> String {
        use super::Type::*;

        match self {
            Void => "void".into(),
            I1 => "i1".into(),
            I32 => "i32".into(),
            Pointer(_) => {
                let elm_typ = func.types.elm_typ(*self);
                format!("*{}", elm_typ.dump(func))
            }
        }
    }
}

impl Constant {
    fn dump(&self) -> String {
        use super::Constant::*;

        match self {
            I32(x) => format!("{}", x),
        }
    }
}
