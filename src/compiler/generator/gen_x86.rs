use crate::compiler::parser::ast::{AstExpression, AstStatement, Function, Operator, Program};

struct Codegen {
    output: String,
    label_num: u32,
}

pub fn generate(program: Program) -> Result<String, String> {
    let mut codegen = Codegen::new();
    codegen.generate(program)
}

impl Codegen {
    fn new() -> Self {
        Codegen {
            output: String::new(),
            label_num: 0,
        }
    }

    fn generate(&mut self, program: Program) -> Result<String, String> {
        self.gen(".intel_syntax noprefix");
        for function in program.functions {
            self.gen_function(function)?;
        }
        Ok(self.output.clone())
    }

    fn gen_function(&mut self, function: Function) -> Result<(), String> {
        self.gen(&format!(".global {}", function.name));
        self.gen_label(&function.name);
        self.gen("  push ebp");
        self.gen("  mov ebp, esp");
        self.gen(format!("  sub ebp, {}", function.ctx.cur_offset).as_str());
        self.gen_statement(&function.body, &function)?;
        Ok(())
    }

    fn gen_statement(&mut self, ast: &AstStatement, function: &Function) -> Result<(), String> {
        match ast {
            AstStatement::Block { stmts } => {
                for stmt in stmts {
                    self.gen_statement(stmt, function)?;
                }
            }
            AstStatement::Declare { name, value } | AstStatement::Assign { name, value } => {
                self.gen_expression(&*value, function)?;
                let variable = function.ctx.find_variable(&name).unwrap();
                self.gen("  pop eax");
                self.gen(format!("  mov [ebp-{}], eax", variable.offset).as_str());
            }
            AstStatement::Return { value } => {
                self.gen_expression(&*value, function)?;
                self.gen("  pop eax");
                self.gen("  pop ebp");
                self.gen("  ret");
            }
            AstStatement::If { cond, then, els } => {
                self.gen_expression(&*cond, function)?;
                self.gen("  pop eax");
                self.gen("  cmp eax, 0");
                let label_else = self.next_label();
                let label_merge = self.next_label();
                self.gen(format!("  je {}", label_else).as_str());

                self.gen_statement(&*then, function)?;
                self.gen(format!("  jmp {}", label_merge).as_str());

                self.gen_label(&label_else);
                if let Some(els) = els {
                    self.gen_statement(&*els, function)?;
                }
                self.gen_label(&label_merge);
            }
        }
        Ok(())
    }

    fn gen_expression(&mut self, ast: &AstExpression, function: &Function) -> Result<(), String> {
        match ast {
            AstExpression::Integer { value } => self.gen_integer(*value),
            AstExpression::Ident { name } => {
                let variable = function.ctx.find_variable(&name).unwrap();
                self.gen(format!("  push [ebp-{}]", variable.offset).as_str());
            }
            AstExpression::BinaryOp { op, lhs, rhs } => {
                self.gen_expression(&*lhs, function)?;
                self.gen_expression(&*rhs, function)?;
                match op {
                    Operator::Add => self.gen_binop("add"),
                    Operator::Sub => self.gen_binop("sub"),
                    Operator::Mul => self.gen_mul(),
                    Operator::Div => self.gen_div(),
                    Operator::And => self.gen_binop("and"),
                    Operator::Or => self.gen_binop("or"),
                    Operator::Xor => self.gen_binop("xor"),
                    Operator::Equal => self.gen_compare("sete"),
                    Operator::NotEqual => self.gen_compare("setne"),
                    Operator::Lt => self.gen_compare("setl"),
                    Operator::Lte => self.gen_compare("setle"),
                    Operator::Gt => self.gen_compare("setg"),
                    Operator::Gte => self.gen_compare("setge"),
                }
            }
        }
        Ok(())
    }

    fn gen_label(&mut self, name: &String) {
        self.gen(&format!("{}:", name));
    }

    fn gen_integer(&mut self, value: u32) {
        self.gen(&format!("  push {}", value));
    }

    fn gen_mul(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  imul ecx");
        self.gen("  push eax");
    }

    fn gen_div(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  xor edx, edx");
        self.gen("  idiv ecx");
        self.gen("  push eax");
    }

    fn gen_binop(&mut self, op: &str) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen(format!("  {} eax, ecx", op).as_str());
        self.gen("  push eax");
    }

    fn gen_compare(&mut self, op: &str) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  cmp eax, ecx");
        self.gen(format!("  {} al", op).as_str());
        self.gen("  push eax");
    }

    fn next_label(&mut self) -> String {
        let label = format!(".L.{}", self.label_num);
        self.label_num += 1;
        label
    }

    fn gen(&mut self, s: &str) {
        self.output.push_str(s);
        self.output.push_str("\n");
    }
}
