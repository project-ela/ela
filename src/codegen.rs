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
        x => return Err(format!("unexpected node: {:?}", x)),
    }
    Ok(())
}

fn gen_integer(value: u32) {
    println!("  push {}", value);
}
