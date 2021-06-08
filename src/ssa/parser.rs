#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub typ: String,
    pub body: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub inst: Vec<Instruction>,
}

#[derive(Debug)]
pub struct Instruction {
    pub dst: Option<Register>,
    pub op: String,
    pub src: Vec<Value>,
}

#[derive(Debug)]
pub struct Value {
    pub typ: String,
    pub kind: ValueKind,
}

#[derive(Debug)]
pub enum ValueKind {
    Register(Register),
    Const(usize),
}

#[derive(Debug)]
pub struct Register {
    pub id: usize,
}

#[derive(Debug, Default)]
struct Context {
    registers: HashMap<usize, ssa::Value>,
}

use std::collections::HashMap;

use crate::ssa;

pub fn parse(input: &str) -> ssa::Module {
    translate(ssa_parser::module(input).unwrap())
}

fn translate(m: Module) -> ssa::Module {
    let mut sm = ssa::Module::new();

    for f in m.functions {
        let sf = trans_func(f, &sm);
        sm.add_function(sf);
    }

    sm
}

fn trans_func(f: Function, sm: &ssa::Module) -> ssa::Function {
    let typ = trans_typ(f.typ);
    let mut sf = ssa::Function::new(sm, f.name, typ, vec![]);
    let mut fb = ssa::FunctionBuilder::new(&mut sf);

    let mut ctx = Context::default();
    for block in f.body {
        trans_block(block, &mut ctx, &mut fb);
    }

    sf
}

fn trans_block(b: Block, ctx: &mut Context, fb: &mut ssa::FunctionBuilder) {
    let block = fb.new_block();
    fb.set_block(block);
    for i in b.inst {
        trans_inst(i, ctx, fb);
    }
}

fn trans_inst(i: Instruction, ctx: &mut Context, fb: &mut ssa::FunctionBuilder) {
    macro_rules! binop {
        ($($op: ident),*) => {
            match i.op.as_str() {
                $(stringify!($op) => {
                    let id = i.dst.unwrap().id;
                    let lhs = trans_value(&i.src[0], ctx);
                    let rhs = trans_value(&i.src[1], ctx);
                    ctx.registers.insert(id, fb.$op(lhs, rhs));
                    return;
                }),*
                _ => {}
            }
        };
    }

    binop!(add, sub, mul, div, rem, shl, shr, and, or, xor, eq, neq, gt, gte, lt, lte);

    match i.op.as_str() {
        "ret" => {
            let v = trans_value(&i.src[0], ctx);
            fb.ret(v)
        }
        _ => unimplemented!(),
    }
}

fn trans_value(v: &Value, ctx: &mut Context) -> ssa::Value {
    match v.kind {
        ValueKind::Const(r#const) => ssa::Value::new_i32(r#const as i32),
        ValueKind::Register(Register { id }) => *ctx.registers.get(&id).unwrap(),
    }
}

fn trans_typ(t: String) -> ssa::Type {
    match t.as_str() {
        "i32" => ssa::Type::I32,
        _ => unimplemented!(),
    }
}

peg::parser! {
     grammar ssa_parser() for str {

        rule _ = [' ' | '\n']*

        pub rule module() -> Module
            = f:function()* {
                Module {
                    functions:f,
                }
            }

        rule function() -> Function
            = "func" _ "@" name:ident() "(" _ ")" _ typ:ident() _"{" _ body:block()* _ "}"{
                Function {
                    name,
                    typ,
                    body,
                }
            }

        rule block() -> Block
            = name:ident() ":" _ inst:inst()* {
                Block {
                    name,
                    inst,
                }
            }

        rule inst() -> Instruction
            =  op:ident() src:(_ src:value() _ {src})** "," {
                Instruction {
                    dst: None,
                    op,
                    src,
                }
            }
            / dst:reg() _ "=" _ op:ident() src:(_ src:value() _ {src}) ** "," {
                Instruction {
                    dst: Some(dst),
                    op,
                    src,
                }
            }

        rule value() -> Value
            = typ:ident() _ reg:reg() {
                Value {
                    typ,
                    kind: ValueKind::Register(reg),
                }
            }
            / typ:ident() _ r#const:number() {
                Value {
                    typ,
                    kind: ValueKind::Const(r#const),
                }
            }

        rule reg() -> Register
            = "%" id:number() {
                Register { id }
            }

        rule ident() -> String
            = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) { s.to_string() }

        rule number() -> usize
            = n:$(['0'..='9']+) {n.parse().unwrap()}
    }
}
