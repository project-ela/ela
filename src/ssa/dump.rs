use super::{BlockId, Function, Immediate, InstructionId, Module, Type, Value};

impl Module {
    pub fn dump(&self) -> String {
        self.functions
            .iter()
            .map(|(_, function)| function.dump())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Function {
    pub fn dump(&self) -> String {
        let block_str = self
            .blocks
            .iter()
            .map(|(block_id, _)| self.dump_block(block_id))
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "func {}() {} {{\n{}\n}}\n",
            self.name,
            self.dump_type(self.ret_typ),
            block_str
        )
    }

    fn dump_block(&self, block_id: BlockId) -> String {
        let block = self.blocks.get(block_id).unwrap();

        let inst_str = block
            .instructions
            .iter()
            .map(|inst_id| self.dump_inst(*inst_id))
            .collect::<Vec<String>>()
            .join("\n");

        format!("  b{}:\n{}", block_id.index(), inst_str)
    }

    fn dump_inst(&self, inst_id: InstructionId) -> String {
        use super::Instruction::*;

        let inst = self.instructions.get(inst_id).unwrap();
        let inst_str = match inst {
            Add(lhs, rhs) => format!("add {}, {}", self.dump_value(lhs), self.dump_value(rhs)),
            Equal(lhs, rhs) => format!("eq {}, {}", self.dump_value(lhs), self.dump_value(rhs)),

            Alloc(typ) => format!("alloc {}", self.dump_type(*typ)),
            Load(src) => format!("load {}", self.dump_value(src)),
            Store(dst, src) => format!("store {}, {}", self.dump_value(dst), self.dump_value(src)),

            Ret(val) => format!("ret {}", self.dump_value(val)),
            Br(dst) => format!("br b{}", dst.index()),
            CondBr(cond, con, alt) => {
                format!(
                    "br {} -> b{} b{} ",
                    self.dump_value(cond),
                    con.index(),
                    alt.index()
                )
            }
        };

        format!("    %{} = {}", inst_id.index(), inst_str)
    }

    fn dump_value(&self, val: &Value) -> String {
        use super::Value::*;

        match val {
            Immediate(imm) => format!("{} {}", self.dump_type(val.typ()), self.dump_immediate(imm)),
            Instruction(inst_val) => {
                format!(
                    "{} %{}",
                    self.dump_type(val.typ()),
                    inst_val.inst_id.index()
                )
            }
        }
    }

    fn dump_type(&self, typ: Type) -> String {
        use super::Type::*;

        match typ {
            Void => "void".into(),
            I1 => "i1".into(),
            I32 => "i32".into(),
            Pointer(_) => {
                let elm_typ = self.types.elm_typ(typ);
                format!("*{}", self.dump_type(elm_typ))
            }
        }
    }

    fn dump_immediate(&self, imm: &Immediate) -> String {
        use super::Immediate::*;

        match imm {
            I32(x) => format!("{}", x),
        }
    }
}
