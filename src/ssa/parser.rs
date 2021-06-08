#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub typ: String,
    pub body: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    O {
        op: String,
        src: Vec<Value>,
    },
    OD {
        dst: Register,
        op: String,
        src: Vec<Value>,
    },
    L {
        name: String,
    },
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
    Label(String),
}

#[derive(Debug)]
pub struct Register {
    pub id: usize,
}

#[derive(Debug, Default)]
struct Context {
    registers: HashMap<usize, ssa::Value>,
    blocks: HashMap<String, ssa::BlockId>,
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

    for inst in &f.body {
        if let Instruction::L { name } = inst {
            ctx.blocks.insert(name.clone(), fb.new_block());
        }
    }

    for inst in f.body {
        trans_inst(inst, &mut ctx, &mut fb);
    }

    sf
}

fn trans_inst(i: Instruction, ctx: &mut Context, fb: &mut ssa::FunctionBuilder) {
    match i {
        Instruction::O { op, src } => match op.as_str() {
            "ret" => {
                let v = trans_value(&src[0], ctx);
                fb.ret(v)
            }
            "br" => {
                let dst = trans_label(&src[0]);
                fb.br(*ctx.blocks.get(dst).unwrap())
            }
            _ => unimplemented!(),
        },
        Instruction::OD { dst, op, src } => {
            let lhs = trans_value(&src[0], ctx);
            let rhs = trans_value(&src[1], ctx);

            macro_rules! binop {
                ($($op: ident),*) => {
                    match op.as_str() {
                        $(stringify!($op) => {
                            ctx.registers.insert(dst.id, fb.$op(lhs, rhs));
                        }),*
                        _ => {}
                    }
                };
            }

            binop!(add, sub, mul, div, rem, shl, shr, and, or, xor, eq, neq, gt, gte, lt, lte);
        }
        Instruction::L { name } => {
            let block = ctx.blocks.get(&name).unwrap();
            fb.set_block(*block);
        }
    }
}

fn trans_value(v: &Value, ctx: &mut Context) -> ssa::Value {
    match v.kind {
        ValueKind::Const(r#const) => ssa::Value::new_i32(r#const as i32),
        ValueKind::Register(Register { id }) => *ctx.registers.get(&id).unwrap(),
        _ => panic!(),
    }
}

fn trans_label(v: &Value) -> &String {
    match v.kind {
        ValueKind::Label(ref l) => l,
        _ => panic!(),
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
            = "func" _ "@" name:ident() "(" _ ")" _ typ:ident() _"{" _ body:inst() ** _ _ "}"{
                Function {
                    name,
                    typ,
                    body,
                }
            }


        rule inst() -> Instruction
            = name:ident() ":" { Instruction::L{name} }
            / op:ident() src:(_ src:value() {src})** "," {
                Instruction::O {
                    op,
                    src,
                }
            }
            / dst:reg() _ "=" _ op:ident() src:(_ src:value() {src}) ** "," {
                Instruction::OD{
                    dst,
                    op,
                    src,
                }
            }

        rule value() -> Value
            = "label" _ name:ident() {
                Value {
                    typ: "label".into(),
                    kind: ValueKind::Label(name),
                }
            }
            / typ:ident() _ r#const:number() {
                Value {
                    typ,
                    kind: ValueKind::Const(r#const),
                }
            }
            / typ:ident() _ reg:reg() {
                Value {
                    typ,
                    kind: ValueKind::Register(reg),
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
