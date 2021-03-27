use super::{BlockId, Function, Immediate, InstructionId, Module, Type, Value};

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
        let block_str = self
            .blocks
            .iter()
            .map(|(block_id, _)| self.dump_block(module, block_id))
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "func {}() {} {{\n{}\n}}\n",
            self.name,
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

            Call(func_id) => format!("call {}", module.function(*func_id).unwrap().name),

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
            Immediate(imm) => format!("{} {}", self.typ().dump(func), imm.dump()),
            Instruction(inst_val) => {
                format!("{} %{}", self.typ().dump(func), inst_val.inst_id.index())
            }
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

impl Immediate {
    fn dump(&self) -> String {
        use super::Immediate::*;

        match self {
            I32(x) => format!("{}", x),
        }
    }
}
