extern crate skuld;

use skuld::{
    backend::generator,
    frontend::{lexer, parser},
};

#[test]
fn ret() {
    do_test("ret", "c3");
}

#[test]
fn push() {
    do_test("push 1", "6a 01");
    do_test("push rax", "50");
    do_test("push r8", "41 50");
}

#[test]
fn pop() {
    do_test("pop rax", "58");
    do_test("pop r8", "41 58");
}

#[test]
fn idiv() {
    do_test("idiv eax", "f7 f8");
    do_test("idiv rax", "48 f7 f8");
    do_test("idiv r8", "49 f7 f8");
}

#[test]
fn jmp() {
    do_test("label: jmp label", "e9 fb ff ff ff"); // skip
}

#[test]
fn je() {
    do_test("label: je label", "0f 84 fa ff ff ff"); // skip
}

#[test]
fn call() {
    do_test("label: call label", "e8 fb ff ff ff");
}

#[test]
fn sete() {
    do_test("sete al", "0f 94 c0");
    do_test("sete r9b", "41 0f 94 c1");
}

#[test]
fn setne() {
    do_test("setne al", "0f 95 c0");
    do_test("setne r9b", "41 0f 95 c1");
}

#[test]
fn setl() {
    do_test("setl al", "0f 9c c0");
    do_test("setl r9b", "41 0f 9c c1");
}

#[test]
fn setle() {
    do_test("setle al", "0f 9e c0");
    do_test("setle r9b", "41 0f 9e c1");
}

#[test]
fn setg() {
    do_test("setg al", "0f 9f c0");
    do_test("setg r9b", "41 0f 9f c1");
}

#[test]
fn setge() {
    do_test("setge al", "0f 9d c0");
    do_test("setge r9b", "41 0f 9d c1");
}

#[test]
fn add() {
    do_test("add eax,1", "83 c0 01");
    do_test("add rax,1", "48 83 c0 01");
    do_test("add r9,1", "49 83 c1 01");
    do_test("add eax,eax", "01 c0");
    do_test("add rax,rax", "48 01 c0");
    do_test("add rax,r9", "4c 01 c8");
    do_test("add r9,rax", "49 01 c1");
    do_test("add r9,r9", "4d 01 c9");
    do_test("add rax, [rax]", "48 03 00");
    do_test("add rax, [r9]", "49 03 01");
    do_test("add [rax], rax", "48 01 00");
    do_test("add [r9], rax", "49 01 01");
    do_test("add rax,[rax+8]", "48 03 40 08");
    do_test("add rax,[r9-8]", "49 03 41 f8");
    do_test("add [rax+8],rax", "48 01 40 08");
    do_test("add [r9-8],rax", "49 01 41 f8");
}

#[test]
fn sub() {
    do_test("sub eax,1", "83 e8 01");
    do_test("sub rax,1", "48 83 e8 01");
    do_test("sub r9,1", "49 83 e9 01");
    do_test("sub eax,eax", "29 c0");
    do_test("sub rax,rax", "48 29 c0");
    do_test("sub rax,r9", "4c 29 c8");
    do_test("sub r9,rax", "49 29 c1");
    do_test("sub r9,r9", "4d 29 c9");
    do_test("sub rax, [rax]", "48 2b 00");
    do_test("sub rax, [r9]", "49 2b 01");
    do_test("sub [rax], rax", "48 29 00");
    do_test("sub [r9], rax", "49 29 01");
    do_test("sub rax,[rax+8]", "48 2b 40 08");
    do_test("sub rax,[r9-8]", "49 2b 41 f8");
    do_test("sub [rax+8],rax", "48 29 40 08");
    do_test("sub [r9-8],rax", "49 29 41 f8");
}

#[test]
fn imul() {
    do_test("imul eax,eax", "0f af c0");
    do_test("imul rax,rax", "48 0f af c0");
    do_test("imul rax,r9", "49 0f af c1");
    do_test("imul r9,rax", "4c 0f af c8");
    do_test("imul r9,r9", "4d 0f af c9");
    do_test("imul rax, [rax]", "48 0f af 00");
    do_test("imul rax, [r9]", "49 0f af 01");
    do_test("imul rax,[rax+8]", "48 0f af 40 08");
    do_test("imul rax,[r9-8]", "49 0f af 41 f8");
}

#[test]
fn xor() {
    do_test("xor eax,1", "83 f0 01");
    do_test("xor rax,1", "48 83 f0 01");
    do_test("xor r9,1", "49 83 f1 01");
    do_test("xor eax,eax", "31 c0");
    do_test("xor rax,rax", "48 31 c0");
    do_test("xor rax,r9", "4c 31 c8");
    do_test("xor r9,rax", "49 31 c1");
    do_test("xor r9,r9", "4d 31 c9");
    do_test("xor rax, [rax]", "48 33 00");
    do_test("xor rax, [r9]", "49 33 01");
    do_test("xor [rax], rax", "48 31 00");
    do_test("xor [r9], rax", "49 31 01");
    do_test("xor rax,[rax+8]", "48 33 40 08");
    do_test("xor rax,[r9-8]", "49 33 41 f8");
    do_test("xor [rax+8],rax", "48 31 40 08");
    do_test("xor [r9-8],rax", "49 31 41 f8");
}

#[test]
fn mov() {
    do_test("mov eax,1", "c7 c0 01 00 00 00"); // skip
    do_test("mov rax,1", "48 c7 c0 01 00 00 00");
    do_test("mov r9,1", "49 c7 c1 01 00 00 00");
    do_test("mov eax,eax", "89 c0");
    do_test("mov rax,rax", "48 89 c0");
    do_test("mov rax,r9", "4c 89 c8");
    do_test("mov r9,rax", "49 89 c1");
    do_test("mov r9,r9", "4d 89 c9");
    do_test("mov rax,[rax]", "48 8b 00");
    do_test("mov rax,[r9]", "49 8b 01");
    do_test("mov [rax],rax", "48 89 00");
    do_test("mov [r9],rax", "49 89 01");
    do_test("mov rax,[rax+8]", "48 8b 40 08");
    do_test("mov rax,[r9-8]", "49 8b 41 f8");
    do_test("mov [rax+8],rax", "48 89 40 08");
    do_test("mov [r9-8],rax", "49 89 41 f8");
}

#[test]
fn and() {
    do_test("and eax,1", "83 e0 01");
    do_test("and rax,1", "48 83 e0 01");
    do_test("and r9,1", "49 83 e1 01");
    do_test("and eax,eax", "21 c0");
    do_test("and rax,rax", "48 21 c0");
    do_test("and rax,r9", "4c 21 c8");
    do_test("and r9,rax", "49 21 c1");
    do_test("and r9,r9", "4d 21 c9");
    do_test("and rax,[rax]", "48 23 00");
    do_test("and rax,[r9]", "49 23 01");
    do_test("and [rax],rax", "48 21 00");
    do_test("and [r9],rax", "49 21 01");
    do_test("and rax,[rax+8]", "48 23 40 08");
    do_test("and rax,[r9-8]", "49 23 41 f8");
    do_test("and [rax+8],rax", "48 21 40 08");
    do_test("and [r9-8],rax", "49 21 41 f8");
}

#[test]
fn or() {
    do_test("or eax,1", "83 c8 01");
    do_test("or rax,1", "48 83 c8 01");
    do_test("or r9,1", "49 83 c9 01");
    do_test("or eax,eax", "09 c0");
    do_test("or rax,rax", "48 09 c0");
    do_test("or rax,r9", "4c 09 c8");
    do_test("or r9,rax", "49 09 c1");
    do_test("or r9,r9", "4d 09 c9");
    do_test("or rax,[rax]", "48 0b 00");
    do_test("or rax,[r9]", "49 0b 01");
    do_test("or [rax],rax", "48 09 00");
    do_test("or [r9],rax", "49 09 01");
    do_test("or rax,[rax+8]", "48 0b 40 08");
    do_test("or rax,[r9-8]", "49 0b 41 f8");
    do_test("or [rax+8],rax", "48 09 40 08");
    do_test("or [r9-8],rax", "49 09 41 f8");
}

#[test]
fn cmp() {
    do_test("cmp eax,1", "83 f8 01");
    do_test("cmp rax,1", "48 83 f8 01");
    do_test("cmp r9,1", "49 83 f9 01");
    do_test("cmp eax,eax", "39 c0");
    do_test("cmp rax,rax", "48 39 c0");
    do_test("cmp rax,r9", "4c 39 c8");
    do_test("cmp r9,rax", "49 39 c1");
    do_test("cmp r9,r9", "4d 39 c9");
    do_test("cmp rax,[rax]", "48 3b 00");
    do_test("cmp rax,[r9]", "49 3b 01");
    do_test("cmp [rax],rax", "48 39 00");
    do_test("cmp [r9],rax", "49 39 01");
    do_test("cmp rax,[rax+8]", "48 3b 40 08");
    do_test("cmp rax,[r9-8]", "49 3b 41 f8");
    do_test("cmp [rax+8],rax", "48 39 40 08");
    do_test("cmp [r9-8],rax", "49 39 41 f8");
}

fn do_test(source: &str, expected_output: &str) {
    let output = lexer::tokenize(source.to_string())
        .and_then(|tokens| parser::parse(tokens))
        .and_then(|insts| generator::generate(insts))
        .unwrap();
    let actual_output = bytes_to_str(&output.program);
    assert_eq!(expected_output, actual_output, "failed with '{}'", source);
}

fn bytes_to_str(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ")
}
