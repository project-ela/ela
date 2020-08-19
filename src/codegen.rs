use crate::ast::AST;

struct Codegen {
    output: String,
    label_num: u32,
}

pub fn generate(ast: AST) -> Result<String, String> {
    let mut codegen = Codegen::new();
    codegen.generate(ast)
}

impl Codegen {
    fn new() -> Self {
        Codegen {
            output: String::new(),
            label_num: 0,
        }
    }

    fn generate(&mut self, ast: AST) -> Result<String, String> {
        self.gen(".intel_syntax noprefix");
        self.gen_function(ast)?;
        Ok(self.output.clone())
    }

    fn gen_function(&mut self, ast: AST) -> Result<(), String> {
        if let AST::Function { name, body } = ast {
            self.gen(&format!(".global {}", name));
            self.gen_label(name);
            self.gen_statement(*body)?;
            Ok(())
        } else {
            Err(format!("expected function, but got {:?}", ast))
        }
    }

    fn gen_statement(&mut self, ast: AST) -> Result<(), String> {
        match ast {
            AST::Return { value } => {
                self.gen_expression(*value)?;
                self.gen("  pop eax");
                self.gen("  ret");
                Ok(())
            }
            AST::If { cond, then } => {
                self.gen_expression(*cond)?;
                self.gen("  pop eax");
                self.gen("  cmp eax, 0");
                let label_then = self.next_label();
                self.gen(format!("  je {}", label_then).as_str());
                self.gen_statement(*then)?;
                self.gen_label(label_then);
                Ok(())
            }
            x => return Err(format!("unexpected node: {:?}", x)),
        }
    }

    fn gen_expression(&mut self, ast: AST) -> Result<(), String> {
        match ast {
            AST::Integer { value } => self.gen_integer(value),
            AST::Add { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_add();
            }
            AST::Sub { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_sub();
            }
            AST::Mul { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_mul();
            }
            AST::Div { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_div();
            }
            AST::Equal { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("sete");
            }
            AST::NotEqual { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setne");
            }
            AST::Lt { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setl");
            }
            AST::Lte { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setle");
            }
            AST::Gt { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setg");
            }
            AST::Gte { lhs, rhs } => {
                self.gen_expression(*lhs)?;
                self.gen_expression(*rhs)?;
                self.gen_compare("setge");
            }
            x => return Err(format!("unexpected node: {:?}", x)),
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
