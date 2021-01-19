use crate::{
    common::{
        error::Error,
        operator::{BinaryOperator, UnaryOperator},
    },
    middleend::irgen::ir::*,
};

const PARAM_REGS: [Register; 6] = [
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R8,
    Register::R9,
];

struct GenX86 {
    output: String,
}

pub fn generate(program: IRProgram) -> Result<String, Error> {
    let mut generator = GenX86::new();
    generator.generate(program)
}

impl GenX86 {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn generate(&mut self, program: IRProgram) -> Result<String, Error> {
        self.gen(".intel_syntax noprefix");
        self.gen_data(program.global_defs)?;
        self.gen_code(program.functions)?;
        Ok(self.output.to_owned())
    }

    fn gen_data(&mut self, global_defs: Vec<IRGlobalDef>) -> Result<(), Error> {
        self.gen(".data");
        for global_def in global_defs {
            self.gen(&format!(".global {}", global_def.name));
            self.gen(&format!("{}:", global_def.name));
            match global_def.init_value {
                Some(value) => self.gen(&format!(".ascii \"{}\"", value)),
                None => self.gen(&format!(".zero {}", global_def.typ.size())),
            }
        }

        Ok(())
    }

    fn gen_code(&mut self, functions: Vec<IRFunction>) -> Result<(), Error> {
        self.gen(".text");
        for function in functions {
            self.gen_function(function)?;
        }
        Ok(())
    }

    fn gen_function(&mut self, function: IRFunction) -> Result<(), Error> {
        self.gen(format!(".global {}", function.name).as_str());
        self.gen(format!("{}:", function.name).as_str());
        self.gen("  push rbp");
        self.gen("  mov rbp, rsp");
        self.gen(&format!("  sub rsp, {}", function.stack_offset));
        self.gen("  push r12");
        self.gen("  push r13");
        self.gen("  push r14");
        self.gen("  push r15");
        for block in function.blocks {
            self.gen(format!("{}:", block.name).as_str());
            for ir in block.irs {
                self.gen_ir(&ir, &function.name)?;
                if is_terminate_inst(&ir) {
                    break;
                }
            }
        }
        self.gen(format!(".L.{}.ret:", function.name).as_str());
        self.gen("  pop r15");
        self.gen("  pop r14");
        self.gen("  pop r13");
        self.gen("  pop r12");
        self.gen("  mov rsp, rbp");
        self.gen("  pop rbp");
        self.gen("  ret");
        Ok(())
    }

    fn gen_ir(&mut self, ir: &IR, func_name: &str) -> Result<(), Error> {
        match ir {
            IR::UnOp { op, src } => match op {
                UnaryOperator::Not => {
                    self.gen(format!("  cmp {}, 0", opr(src)).as_str());
                    self.gen(format!("  sete {}", opr8(src)).as_str());
                }
                _ => panic!(),
            },
            IR::BinOp { op, dst, lhs, rhs } => {
                // r0 = r1 <op> r2 -> r1 = r0; r1 = r1 <op> r2
                if !dst.is_same(lhs) {
                    self.gen(format!("  mov {}, {}", opr(dst), opr(lhs)).as_str());
                }

                match op {
                    BinaryOperator::Add => self.gen_binop("add", dst, rhs),
                    BinaryOperator::Sub => self.gen_binop("sub", dst, rhs),
                    BinaryOperator::Mul => self.gen_binop("imul", dst, rhs),
                    BinaryOperator::Div => self.gen_div(dst, rhs),
                    BinaryOperator::Mod => self.gen_mod(dst, rhs),
                    BinaryOperator::And => self.gen_binop("and", dst, rhs),
                    BinaryOperator::Or => self.gen_binop("or", dst, rhs),
                    BinaryOperator::Xor => self.gen_binop("xor", dst, rhs),
                    BinaryOperator::Equal => self.gen_compare("sete", dst, rhs),
                    BinaryOperator::NotEqual => self.gen_compare("setne", dst, rhs),
                    BinaryOperator::Lt => self.gen_compare("setl", dst, rhs),
                    BinaryOperator::Lte => self.gen_compare("setle", dst, rhs),
                    BinaryOperator::Gt => self.gen_compare("setg", dst, rhs),
                    BinaryOperator::Gte => self.gen_compare("setge", dst, rhs),
                }
            }
            IR::Call { dst, name, args } => {
                for reg in PARAM_REGS.iter().take(args.len()) {
                    self.gen(format!("  push {}", reg.dump()).as_str());
                }
                self.gen_args(&args);
                self.gen(format!("  call {}", name).as_str());
                for reg in PARAM_REGS.iter().take(args.len()).rev() {
                    self.gen(format!("  pop {}", reg.dump()).as_str());
                }
                if dst.is_some() {
                    self.gen(format!("  mov {}, rax", opr(&dst.unwrap())).as_str());
                }
            }
            IR::Addr { dst, src } => self.gen(&format!("  lea {}, [rbp{:+}]", opr(dst), src)),
            IR::AddrLabel { dst, src } => self.gen(&format!("  lea {}, [rip+{}]", opr(dst), src)),
            IR::Move { dst, src } => self.gen(format!("  mov {}, {}", opr(dst), opr(src)).as_str()),
            IR::Load { dst, src, size } => match size {
                RegSize::Byte => {
                    self.gen(&format!("  movsx {}, byte ptr [{}]", opr(dst), opr(src)))
                }
                RegSize::QWord => self.gen(&format!("  mov {}, [{}]", opr(dst), opr(src))),
                _ => unimplemented!(),
            },
            IR::Store { dst, src, size } => match size {
                RegSize::Byte => self.gen(&format!("  mov [{}], {}", opr(dst), opr8(src))),
                RegSize::QWord => self.gen(&format!("  mov [{}], {}", opr(dst), opr(src))),
                _ => unimplemented!(),
            },
            IR::StoreArg {
                dst,
                src: param_index,
                size,
            } => {
                let param_reg = PARAM_REGS[*param_index];
                match size {
                    RegSize::Byte => {
                        self.gen(&format!("  mov [rbp{:+}], {}", dst, reg8(param_reg)))
                    }
                    RegSize::QWord => {
                        self.gen(&format!("  mov [rbp{:+}], {}", dst, reg(&param_reg)))
                    }
                    _ => unimplemented!(),
                }
            }

            IR::Jump { label } => self.gen(format!("  jmp {}", label).as_str()),
            IR::JumpIfNot { label, cond } => {
                self.gen(format!("  cmp {}, 0", opr(cond)).as_str());
                self.gen(format!("  je {}", label).as_str());
            }
            IR::Ret { src } => {
                if let Some(src) = src {
                    self.gen(format!("  mov rax, {}", opr(src)).as_str());
                }
                self.gen(format!("  jmp .L.{}.ret", func_name).as_str());
            }
        }
        Ok(())
    }

    fn gen_args(&mut self, args: &[Operand]) {
        for (arg, reg) in (&args).iter().zip(&PARAM_REGS) {
            self.gen(format!("  mov {}, {}", reg.dump(), opr(arg)).as_str());
        }
        // TODO: when args.len() > 6
    }

    fn gen_binop(&mut self, op: &str, lhs: &Operand, rhs: &Operand) {
        self.gen(format!("  {} {}, {}", op, opr(lhs), opr(rhs)).as_str())
    }

    fn gen_div(&mut self, lhs: &Operand, rhs: &Operand) {
        self.gen(format!("  mov rax, {}", opr(lhs)).as_str());
        self.gen(format!("  mov rcx, {}", opr(rhs)).as_str());
        self.gen("  xor rdx, rdx");
        self.gen("  idiv rcx");
        self.gen(format!("  mov {}, rax", opr(lhs)).as_str());
    }

    fn gen_mod(&mut self, lhs: &Operand, rhs: &Operand) {
        self.gen(format!("  mov rax, {}", opr(lhs)).as_str());
        self.gen(format!("  mov rcx, {}", opr(rhs)).as_str());
        self.gen("  xor rdx, rdx");
        self.gen("  idiv rcx");
        self.gen(format!("  mov {}, rdx", opr(lhs)).as_str());
    }

    fn gen_compare(&mut self, op: &str, lhs: &Operand, rhs: &Operand) {
        self.gen(format!("  cmp {}, {}", opr(lhs), opr(rhs)).as_str());
        self.gen(format!("  {} {}", op, opr8(lhs)).as_str());
    }

    fn gen(&mut self, s: &str) {
        self.output.push_str(s);
        self.output.push_str("\n");
    }
}

fn is_terminate_inst(ir: &IR) -> bool {
    matches!(ir, IR::Ret {.. }|IR::Jump {..})
}

fn opr(operand: &Operand) -> String {
    match operand {
        Operand::Const(value) => format!("{}", value),
        Operand::Reg(info) => reg(&info.physical_index.unwrap()).to_string(),
    }
}

fn opr8(operand: &Operand) -> String {
    match operand {
        Operand::Reg(reg) => reg8(reg.physical_index.unwrap()).to_owned(),
        _ => unreachable!(),
    }
}

fn reg(reg: &Register) -> &'static str {
    match reg {
        Register::Rax => "rax",
        Register::Rbx => "rbx",
        Register::Rcx => "rcx",
        Register::Rdx => "rdx",
        Register::Rdi => "rdi",
        Register::Rsi => "rsi",
        Register::Rbp => "rbp",
        Register::Rsp => "rsp",

        Register::R8 => "r8",
        Register::R9 => "r9",
        Register::R10 => "r10",
        Register::R11 => "r11",
        Register::R12 => "r12",
        Register::R13 => "r13",
        Register::R14 => "r14",
        Register::R15 => "r15",
    }
}

fn reg8(reg: Register) -> &'static str {
    match reg {
        Register::Rax => "al",
        Register::Rcx => "cl",
        Register::Rdx => "dl",
        Register::Rbx => "bl",
        Register::Rdi => "dil",
        Register::Rsi => "sil",
        Register::Rbp => "bpl",
        Register::Rsp => "spl",

        Register::R8 => "r8b",
        Register::R9 => "r9b",
        Register::R10 => "r10b",
        Register::R11 => "r11b",
        Register::R12 => "r12b",
        Register::R13 => "r13b",
        Register::R14 => "r14b",
        Register::R15 => "r15b",
    }
}
