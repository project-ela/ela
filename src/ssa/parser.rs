#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub typ: Type,
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
    OT {
        dst: Register,
        op: String,
        typ: Type,
    },
    L {
        name: String,
    },
}

#[derive(Debug)]
pub struct Value {
    pub typ: Type,
    pub kind: ValueKind,
}

#[derive(Debug)]
pub enum ValueKind {
    Register(Register),
    Const(usize),
    Label(String),
    Zero,
}

#[derive(Debug)]
pub struct Register {
    pub id: usize,
}

#[derive(Debug)]
pub enum Type {
    Void,

    I1,
    I32,
    Pointer(Box<Type>),
    Array(usize, Box<Type>),
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
    let typ = {
        let mut types = sm.types.borrow_mut();
        trans_typ(f.typ, &mut types)
    };
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
            "store" => {
                let dst = trans_value(&src[0], ctx);
                let src = trans_value(&src[1], ctx);
                fb.store(dst, src);
            }
            "ret" => {
                let v = trans_value(&src[0], ctx);
                fb.ret(v);
            }
            "br" => match src.len() {
                1 => {
                    let dst = trans_label(&src[0], ctx);
                    fb.br(dst);
                }
                3 => {
                    let cond = trans_value(&src[0], ctx);
                    let con = trans_label(&src[1], ctx);
                    let alt = trans_label(&src[2], ctx);
                    fb.cond_br(cond, con, alt);
                }
                _ => panic!(),
            },
            _ => unimplemented!(),
        },
        Instruction::OD { dst, op, src } => {
            match op.as_str() {
                "load" => {
                    let src = trans_value(&src[0], ctx);
                    ctx.registers.insert(dst.id, fb.load(src));
                    return;
                }
                "gep" => {
                    let val = trans_value(&src[0], ctx);
                    let indices = src[1..].iter().map(|v| trans_value(v, ctx)).collect();
                    ctx.registers.insert(dst.id, fb.gep(val, indices));
                    return;
                }
                _ => {}
            }

            let lhs = trans_value(&src[0], ctx);
            let rhs = trans_value(&src[1], ctx);

            macro_rules! binop {
                ($($op: ident),*) => {
                    match op.as_str() {
                        $(stringify!($op) => {
                            ctx.registers.insert(dst.id, fb.$op(lhs, rhs));
                        }),*
                        _ => panic!(),
                    }
                };
            }

            binop!(add, sub, mul, div, rem, shl, shr, and, or, xor, eq, neq, gt, gte, lt, lte);
        }
        Instruction::OT { dst, op, typ } => match op.as_str() {
            "alloc" => {
                let typ = {
                    let mut types = fb.function_mut().types.borrow_mut();
                    trans_typ(typ, &mut types)
                };
                ctx.registers.insert(dst.id, fb.alloc(typ));
            }
            _ => panic!(),
        },
        Instruction::L { name } => {
            let block = ctx.blocks.get(&name).unwrap();
            fb.set_block(*block);
        }
    }
}

fn trans_value(v: &Value, ctx: &Context) -> ssa::Value {
    match v.kind {
        ValueKind::Const(r#const) => ssa::Value::new_i32(r#const as i32),
        ValueKind::Register(Register { id }) => *ctx.registers.get(&id).unwrap(),
        ValueKind::Zero => ssa::Value::new_zero(),
        _ => panic!(),
    }
}

fn trans_label(v: &Value, ctx: &Context) -> ssa::BlockId {
    match v.kind {
        ValueKind::Label(ref l) => *ctx.blocks.get(l).unwrap(),
        _ => panic!(),
    }
}

fn trans_typ(t: Type, types: &mut ssa::Types) -> ssa::Type {
    match t {
        Type::I1 => ssa::Type::I1,
        Type::I32 => ssa::Type::I32,
        Type::Pointer(elm) => {
            let elm = trans_typ(*elm, types);
            types.ptr_to(elm)
        }
        Type::Array(len, elm) => {
            let elm = trans_typ(*elm, types);
            types.array_of(elm, len)
        }
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
            = "func" _ "@" name:ident() "(" _ ")" _ typ:comp_typ() _"{" _ body:inst() ** _ _ "}"{
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
            / dst:reg() _ "=" _ "alloc" _ typ:comp_typ() {
                Instruction::OT {
                    dst,
                    op: "alloc".into(),
                    typ,
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
                    typ: Type::Void,
                    kind: ValueKind::Label(name),
                }
            }
            / typ:comp_typ() _ r#const:number() {
                Value {
                    typ,
                    kind: ValueKind::Const(r#const),
                }
            }
            / typ:comp_typ() _ reg:reg() {
                Value {
                    typ,
                    kind: ValueKind::Register(reg),
                }
            }
            / typ:comp_typ() _ "zero" {
                Value {
                    typ,
                    kind: ValueKind::Zero,
                }
            }

        rule reg() -> Register
            = "%" id:number() {
                Register { id }
            }

        rule comp_typ() -> Type
            = "*" _ elm:comp_typ() { Type::Pointer(Box::new(elm)) }
            / "[" _ len:number() _ "]" _ elm:comp_typ() { Type::Array(len, Box::new(elm)) }
            / elm:typ() { elm }

        rule typ() -> Type
            = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '*']+) {
                match s {
                    "i1" => Type::I1,
                    "i32" => Type::I32,
                    _ => unimplemented!(),
                }
             }

        rule ident() -> String
            = s:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) { s.to_string() }

        rule number() -> usize
            = n:$(['0'..='9']+) {n.parse().unwrap()}
    }
}
