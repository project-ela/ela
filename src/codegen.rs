use crate::ast::*;

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
        self.gen_label(function.name);
        self.gen_statement(function.body)?;
        Ok(())
    }

    fn gen_statement(&mut self, ast: AstStatement) -> Result<(), String> {
        match ast {
            AstStatement::Block { stmts } => {
                for stmt in stmts {
                    self.gen_statement(stmt)?;
                }
                Ok(())
            }
            AstStatement::Return { value } => {
                self.gen_expression(*value)?;
                self.gen("  pop eax");
                self.gen("  ret");
                Ok(())
            }
            AstStatement::If { cond, then, els } => {
                self.gen_expression(*cond)?;
                self.gen("  pop eax");
                self.gen("  cmp eax, 0");
                let label_else = self.next_label();
                let label_merge = self.next_label();
                self.gen(format!("  je {}", label_else).as_str());

                self.gen_statement(*then)?;
                self.gen(format!("  jmp {}", label_merge).as_str());

                self.gen_label(label_else);
                if let Some(els) = els {
                    self.gen_statement(*els)?;
                }
                self.gen_label(label_merge);
                Ok(())
            }
        }
    }

    fn gen_expression(&mut self, ast: AstExpression) -> Result<(), String> {
        match ast {
            AstExpression::Integer { value } => self.gen_integer(value),
            AstExpression::Add { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_add();
            }
            AstExpression::Sub { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_sub();
            }
            AstExpression::Mul { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_mul();
            }
            AstExpression::Div { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_div();
            }
            AstExpression::Equal { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("sete");
            }
            AstExpression::NotEqual { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setne");
            }
            AstExpression::Lt { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setl");
            }
            AstExpression::Lte { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setle");
            }
            AstExpression::Gt { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setg");
            }
            AstExpression::Gte { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setge");
            }
        }
        Ok(())
    }

    fn gen_label(&mut self, name: String) {
        self.gen(&format!("{}:", name));
    }

    fn gen_integer(&mut self, value: u32) {
        self.gen(&format!("  push {}", value));
    }

    fn gen_add(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  add eax, ecx");
        self.gen("  push eax");
    }

    fn gen_sub(&mut self) {
        self.gen("  pop ecx");
        self.gen("  pop eax");
        self.gen("  sub eax, ecx");
        self.gen("  push eax");
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
