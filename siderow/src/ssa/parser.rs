#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub typ: Type,
    pub params: Vec<Type>,
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
    Call {
        dst: Option<Register>,
        name: String,
        args: Vec<Value>,
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
    I8,
    I32,

    Pointer(Box<Type>),
    Array(usize, Box<Type>),
}

#[derive(Debug, Default)]
struct Context {
    functions: HashMap<String, ssa::FunctionId>,
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
    let mut ctx = Context::default();

    for f in m.functions {
        let name = f.name.clone();
        let sf = trans_func(f, &sm, &mut ctx);
        ctx.functions.insert(name, sm.add_function(sf));
    }

    sm
}

fn trans_func(f: Function, sm: &ssa::Module, ctx: &mut Context) -> ssa::Function {
    let (ret_typ, param_typ) = {
        let ret_typ = trans_typ(f.typ);
        let param_typ = f.params.into_iter().map(|param| trans_typ(param)).collect();

        (ret_typ, param_typ)
    };

    let mut sf = ssa::Function::new(f.name, ret_typ, param_typ);
    for i in 0..sf.param_typ.len() {
        ctx.registers.insert(i, ssa::Value::new_param(&sf, i));
    }

    let mut fb = ssa::FunctionBuilder::new(&mut sf);

    for inst in &f.body {
        if let Instruction::L { name } = inst {
            ctx.blocks.insert(name.clone(), fb.new_block());
        }
    }

    for inst in f.body {
        trans_inst(inst, sm, ctx, &mut fb);
    }

    sf
}

fn trans_inst(i: Instruction, sm: &ssa::Module, ctx: &mut Context, fb: &mut ssa::FunctionBuilder) {
    match i {
        Instruction::O { op, src } => match op.as_str() {
            "store" => {
                let dst = trans_value(&src[0], ctx);
                let src = trans_value(&src[1], ctx);
                fb.store(dst, src);
            }
            "ret" => match src.len() {
                0 => fb.ret_void(),
                1 => {
                    let v = trans_value(&src[0], ctx);
                    fb.ret(v);
                }
                _ => panic!(),
            },
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
                let typ = trans_typ(typ);
                ctx.registers.insert(dst.id, fb.alloc(typ));
            }
            _ => panic!(),
        },
        Instruction::Call { dst, name, args } => {
            let func_id = ctx.functions.get(&name).unwrap();
            let args = args.iter().map(|v| trans_value(&v, ctx)).collect();
            let ret_val = fb.call(sm, *func_id, args);
            if let Some(dst) = dst {
                ctx.registers.insert(dst.id, ret_val);
            }
        }
        Instruction::L { name } => {
            let block = ctx.blocks.get(&name).unwrap();
            fb.set_block(*block);
        }
    }
}

fn trans_value(v: &Value, ctx: &Context) -> ssa::Value {
    match v.kind {
        ValueKind::Const(r#const) => ssa::Value::new_i32(r#const as i32),
        ValueKind::Register(Register { id }) => ctx.registers.get(&id).unwrap().clone(),
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

fn trans_typ(t: Type) -> ssa::Type {
    match t {
        Type::Void => ssa::Type::Void,
        Type::I1 => ssa::Type::I1,
        Type::I8 => ssa::Type::I8,
        Type::I32 => ssa::Type::I32,
        Type::Pointer(elm) => trans_typ(*elm).ptr_to(),
        Type::Array(len, elm) => trans_typ(*elm).array_of(len),
    }
}

peg::parser! {
     grammar ssa_parser() for str {

        rule _ = [' ' | '\n']*

        pub rule module() -> Module
            = f:function() ** _ {
                Module {
                    functions:f,
                }
            }

        rule function() -> Function
            = "func" _ "@" name:ident() _ "(" _ params:params() _ ")" _ typ:comp_typ() _"{" _ body:inst() ** _ _ "}"{
                Function {
                    name,
                    typ,
                    params,
                    body,
                }
            }

        rule params() -> Vec<Type>
            = (_ typ:comp_typ() {typ}) ** ","

        rule inst() -> Instruction
            = name:ident() ":" { Instruction::L{name} }
            / dst:(dst:reg() _ "=" { dst })? _ "call" _ "@" name:ident() _ "(" args:values() _ ")" {
                Instruction::Call {
                    dst,
                    name,
                    args,
                }
            }
            / op:ident() src:values() {
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
            / dst:reg() _ "=" _ op:ident() src:values() {
                Instruction::OD{
                    dst,
                    op,
                    src,
                }
            }


        rule values() -> Vec<Value>
            = (_ value:value() {value}) ** ","

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
                    "void" => Type::Void,
                    "i1" => Type::I1,
                    "i8" => Type::I8,
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
