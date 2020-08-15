use crate::ast::AST;

pub fn generate(ast: AST) -> Result<(), String> {
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    gen_expression(ast)?;
    println!("  pop eax");
    println!("  ret");

    Ok(())
}

fn gen_expression(ast: AST) -> Result<(), String> {
    match ast {
        AST::Integer { value } => gen_integer(value),
        AST::Add { lhs, rhs } => {
            gen_expression(*lhs)?;
            gen_expression(*rhs)?;
            gen_add();
        }
        AST::Sub { lhs, rhs } => {
            gen_expression(*lhs)?;
            gen_expression(*rhs)?;
            gen_sub();
        }
    }
    Ok(())
}
fn gen_integer(value: u32) {
    println!("  push {}", value);
}

fn gen_add() {
    println!("  pop ecx");
    println!("  pop eax");
    println!("  add eax, ecx");
    println!("  push eax");
}

fn gen_sub() {
    println!("  pop ecx");
    println!("  pop eax");
    println!("  sub eax, ecx");
    println!("  push eax");
}
