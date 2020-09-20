use crate::{
    common::operator::{BinaryOperator, UnaryOperator},
    middleend::tacgen::tac::*,
};

struct GenX86 {
    output: String,
}

pub fn generate(program: TacProgram) -> Result<String, String> {
    let mut generator = GenX86::new();
    generator.generate(program)
}

impl GenX86 {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn generate(&mut self, program: TacProgram) -> Result<String, String> {
        self.gen(".intel_syntax noprefix");
        for function in program.functions {
            self.gen_function(function)?;
        }
        Ok(self.output.to_owned())
    }

    fn gen_function(&mut self, funciton: TacFunction) -> Result<(), String> {
        self.gen(format!(".global {}", funciton.name).as_str());
        self.gen(format!("{}:", funciton.name).as_str());
        self.gen("  push ebp");
        self.gen("  mov ebp, esp");
        self.gen("  push ecx");
        self.gen("  push edx");
        self.gen("  push ebx");
        for tac in funciton.body {
            self.gen_tac(tac, &funciton.name)?;
        }
        self.gen(format!(".L.{}.ret:", funciton.name).as_str());
        self.gen("  pop ebx");
        self.gen("  pop edx");
        self.gen("  pop ecx");
        self.gen("  mov esp, ebp");
        self.gen("  pop ebp");
        self.gen("  ret");
        Ok(())
    }

    fn gen_tac(&mut self, tac: Tac, func_name: &String) -> Result<(), String> {
        match tac {
            Tac::Label { index } => self.gen(format!(".L.{}:", index).as_str()),
            Tac::UnOp { op, src } => match op {
                UnaryOperator::Not => {
                    self.gen(format!("  cmp {}, 0", opr(&src)).as_str());
                    self.gen(format!("  sete {}", opr8(&src)).as_str());
                }
            },
            Tac::BinOp { op, dst, lhs, rhs } => {
                // r0 = r1 <op> r2 -> r1 = r0; r1 = r1 <op> r2
                self.gen(format!("  mov {}, {}", opr(&dst), opr(&lhs)).as_str());

                match op {
                    BinaryOperator::Add => self.gen_binop("add", dst, rhs),
                    BinaryOperator::Sub => self.gen_binop("sub", dst, rhs),
                    BinaryOperator::Mul => self.gen_binop("imul", dst, rhs),
                    BinaryOperator::Div => self.gen_div(dst, rhs),
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
            Tac::Call { dst, name } => match dst {
                Some(dst) => {
                    let mut is_eax = false;
                    if let Operand::Reg(reg) = &dst {
                        if reg.physical_index.unwrap() == Register::Eax {
                            is_eax = true;
                        }
                    }
                    if !is_eax {
                        self.gen("  push eax");
                    }
                    self.gen(format!("  call {}", name).as_str());
                    self.gen(format!("  mov {}, eax", opr(&dst)).as_str());
                    if !is_eax {
                        self.gen("  pop eax");
                    }
                }
                None => self.gen(format!("  call {}", name).as_str()),
            },
            Tac::Move { dst, src } => {
                self.gen(format!("  mov {}, {}", opr(&dst), opr(&src)).as_str())
            }
            Tac::Jump { label_index } => self.gen(format!("  jmp .L.{}", label_index).as_str()),
            Tac::JumpIfNot { label_index, cond } => {
                self.gen(format!("  cmp {}, 0", opr(&cond)).as_str());
                self.gen(format!("  je .L.{}", label_index).as_str());
            }
            Tac::Ret { src } => {
                if let Some(src) = src {
                    self.gen(format!("  mov eax, {}", opr(&src)).as_str());
                }
                self.gen(format!("  jmp .L.{}.ret", func_name).as_str());
            }
        }
        Ok(())
    }

    fn gen_binop(&mut self, op: &str, lhs: Operand, rhs: Operand) {
        self.gen(format!("  {} {}, {}", op, opr(&lhs), opr(&rhs)).as_str())
    }

    fn gen_div(&mut self, lhs: Operand, rhs: Operand) {
        let mut is_eax = false;
        if let Operand::Reg(reg) = &lhs {
            if reg.physical_index.unwrap() == Register::Eax {
                is_eax = true;
            }
        }
        if !is_eax {
            self.gen("  push eax");
        }
        self.gen("  push ecx");
        self.gen("  push edx");
        self.gen(format!("  mov eax, {}", opr(&lhs)).as_str());
        self.gen(format!("  mov ecx, {}", opr(&rhs)).as_str());
        self.gen("  xor edx, edx");
        self.gen("  idiv ecx");
        self.gen(format!("  mov {}, eax", opr(&lhs)).as_str());
        self.gen("  pop edx");
        self.gen("  pop ecx");
        if !is_eax {
            self.gen("  pop eax");
        }
    }

    fn gen_compare(&mut self, op: &str, lhs: Operand, rhs: Operand) {
        self.gen(format!("  cmp {}, {}", opr(&lhs), opr(&rhs)).as_str());
        self.gen(format!("  {} {}", op, opr8(&lhs)).as_str());
    }

    fn gen(&mut self, s: &str) {
        self.output.push_str(s);
        self.output.push_str("\n");
    }
}

fn opr(operand: &Operand) -> String {
    match operand {
        Operand::Const(value) => format!("{}", value),
        Operand::Reg(info) => reg(&info.physical_index.unwrap()).to_string(),
        Operand::Variable(offset) => format!("[ebp-{}]", offset),
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
        Register::Eax => "eax",
        Register::Ecx => "ecx",
        Register::Edx => "edx",
        Register::Ebx => "ebx",
    }
}

fn reg8(reg: Register) -> &'static str {
    match reg {
        Register::Eax => "al",
        Register::Ecx => "cl",
        Register::Edx => "dl",
        Register::Ebx => "bl",
    }
}
