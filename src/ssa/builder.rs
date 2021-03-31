use super::{
    BinaryOperator, Block, BlockId, ComparisonOperator, Function, FunctionId, InstructionKind,
    Module, Terminator, Type, Value,
};

#[derive(Debug)]
pub struct FunctionBuilder<'a> {
    pub function: &'a mut Function,

    pub current_block: Option<BlockId>,
}

impl<'a> FunctionBuilder<'a> {
    pub fn new(function: &'a mut Function) -> Self {
        Self {
            function,
            current_block: None,
        }
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn add_block(&mut self) -> BlockId {
        self.function.add_block()
    }

    pub fn set_block(&mut self, block_id: BlockId) {
        self.current_block = Some(block_id);
    }

    pub fn current_block(&mut self) -> &mut Block {
        let block_id = self.current_block.unwrap();
        self.function.block_mut(block_id).unwrap()
    }

    fn add_inst(&mut self, inst_kind: InstructionKind, typ: Type) -> Value {
        let inst_id = self.function.add_inst(inst_kind);
        self.current_block().add_inst(inst_id);
        Value::new_inst(inst_id, typ)
    }

    fn add_term(&mut self, term: Terminator) {
        let term_id = self.function.add_term(term);
        self.current_block().set_term(term_id);
    }

    pub fn call(&mut self, module: &Module, func_id: FunctionId, args: Vec<Value>) -> Value {
        let ret_typ = module.function(func_id).unwrap().ret_typ;
        self.add_inst(InstructionKind::Call(func_id, args), ret_typ)
    }

    pub fn alloc(&mut self, typ: Type) -> Value {
        let ptr_typ = self.function.types.ptr_to(typ);
        self.add_inst(InstructionKind::Alloc(typ), ptr_typ)
    }

    pub fn load(&mut self, module: &Module, src: Value) -> Value {
        let elm_typ = match src {
            Value::Global(_) | Value::Parameter(_) => module.types.elm_typ(src.typ()),
            _ => self.function.types.elm_typ(src.typ()),
        };
        self.add_inst(InstructionKind::Load(src), elm_typ)
    }

    pub fn store(&mut self, dst: Value, src: Value) {
        let inst_id = self.function.add_inst(InstructionKind::Store(dst, src));
        self.current_block().add_inst(inst_id);
    }

    pub fn ret_void(&mut self) {
        self.add_term(Terminator::Ret(None));
    }

    pub fn ret(&mut self, val: Value) {
        self.add_term(Terminator::Ret(Some(val)));
    }

    pub fn br(&mut self, dst: BlockId) {
        self.add_term(Terminator::Br(dst));
    }

    pub fn cond_br(&mut self, cond: Value, con: BlockId, alt: BlockId) {
        self.add_term(Terminator::CondBr(cond, con, alt));
    }
}

macro_rules! binop {
    ($name: tt, $op: tt) => {
        impl<'a> FunctionBuilder<'a> {
            pub fn $name(&mut self, lhs: Value, rhs: Value) -> Value {
                let inst_kind = InstructionKind::BinOp(BinaryOperator::$op, lhs, rhs);
                self.add_inst(inst_kind, lhs.typ())
            }
        }
    };
}

macro_rules! cmp {
    ($name: tt, $op: tt) => {
        impl<'a> FunctionBuilder<'a> {
            pub fn $name(&mut self, lhs: Value, rhs: Value) -> Value {
                let inst_kind = InstructionKind::Cmp(ComparisonOperator::$op, lhs, rhs);
                self.add_inst(inst_kind, Type::I1)
            }
        }
    };
}

binop!(add, Add);
binop!(sub, Add);
binop!(mul, Add);
binop!(div, Add);
binop!(rem, Add);
binop!(shl, Add);
binop!(shr, Add);
binop!(and, Add);
binop!(or, Add);
binop!(xor, Add);

cmp!(eq, Eq);
cmp!(neq, Neq);
cmp!(gt, Gt);
cmp!(gte, Gte);
cmp!(lt, Lt);
cmp!(lte, Lte);
