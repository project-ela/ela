use crate::{arch::x86::asm, ssa};

pub fn register_size(typ: &ssa::Type) -> asm::RegisterSize {
    use asm::RegisterSize::*;
    use ssa::Type::*;

    match typ {
        I1 | I8 => Byte,
        I32 => QWord,

        Pointer(_) | Array(_, _) => QWord,
        x => panic!("{:?}", x),
    }
}

pub fn type_size_in_bits(typ: &ssa::Type) -> usize {
    use ssa::Type::*;

    match typ {
        Void => 0,
        I1 | I8 => 1,
        I32 => 8,

        Pointer(_) => 8,
        Array(elm_typ, len) => type_size_in_bits(elm_typ) * len,
    }
}


pub(crate) fn align_to(x: usize, align: usize) -> usize {
    (x + align - 1) & !(align - 1)
}
