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

    for block in f.body {
        trans_block(block, &mut fb);
    }

    sf
}

fn trans_block(b: Block, fb: &mut ssa::FunctionBuilder) {
    let block = fb.new_block();
    fb.set_block(block);
    for i in b.inst {
        trans_inst(i, fb);
    }
}

fn trans_inst(i: Instruction, fb: &mut ssa::FunctionBuilder) {
    match i.op.as_str() {
        "ret" => {
            let v = trans_value(&i.src[0], fb);
            fb.ret(v)
        }
        _ => panic!(),
    }
}

fn trans_value(v: &Value, _fb: &mut ssa::FunctionBuilder) -> ssa::Value {
    match v.kind {
        ValueKind::Const(r#const) => ssa::Value::new_i32(r#const as i32),
        _ => panic!(),
    }
}

fn trans_typ(t: String) -> ssa::Type {
    match t.as_str() {
        "i32" => ssa::Type::I32,
        _ => panic!(),
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
