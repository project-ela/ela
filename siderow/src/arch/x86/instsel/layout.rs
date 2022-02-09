use crate::{arch::x86::asm, ssa};

pub fn register_size(typ: &ssa::Type) -> asm::RegisterSize {
    use asm::RegisterSize::*;
    use ssa::Type::*;

    match typ {
        I1 | I8 => Byte,
        I32 => QWord,

        Pointer(_) | Array(_, _) => QWord,
        Structure(_) => QWord, // TODO

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
        Structure(typ) => struct_size_in_bits(typ),
    }
}

pub fn struct_size_in_bits(typ: &ssa::StructureType) -> usize {
    let mut total_size: usize = 0;
    for member in &typ.members {
        let align = register_size(member).size_in_bits();
        let member_size = type_size_in_bits(member);
        total_size = align_to(total_size, align) + member_size;
    }
    total_size
}

pub fn member_offset_in_bits(typ: &ssa::Type, index: usize) -> usize {
    use ssa::Type::*;

    match typ {
        Pointer(elm_typ) => type_size_in_bits(elm_typ) * index,
        Array(elm_typ, _) => type_size_in_bits(elm_typ) * index,
        Structure(s) => {
            let mut total_offet = 0;
            for i in 0..index {
                let align = register_size(&s.members[i + 1]).size_in_bits();
                let member_size = type_size_in_bits(&s.members[i]);
                total_offet = align_to(total_offet + member_size, align);
            }
            total_offet
        }

        x => panic!("{:?}", x),
    }
}

pub(crate) fn align_to(x: usize, align: usize) -> usize {
    (x + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use crate::ssa;

    #[test]
    fn member_offset_in_bits_pointer() {
        use super::member_offset_in_bits;

        let typ = ssa::Type::I8.ptr_to();

        assert_eq!(member_offset_in_bits(&typ, 0), 0);
        assert_eq!(member_offset_in_bits(&typ, 1), 1);
        assert_eq!(member_offset_in_bits(&typ, 2), 2);
    }

    #[test]
    fn member_offset_in_bits_array() {
        use super::member_offset_in_bits;

        let typ = ssa::Type::I32.array_of(4);

        assert_eq!(member_offset_in_bits(&typ, 0), 0);
        assert_eq!(member_offset_in_bits(&typ, 1), 8);
        assert_eq!(member_offset_in_bits(&typ, 2), 16);
    }

    #[test]
    fn member_offset_in_bits_struct1() {
        use super::member_offset_in_bits;

        let typ = ssa::Type::Structure(ssa::StructureType {
            members: vec![ssa::Type::I32, ssa::Type::I8, ssa::Type::I32],
        });

        assert_eq!(member_offset_in_bits(&typ, 0), 0);
        assert_eq!(member_offset_in_bits(&typ, 1), 8);
        assert_eq!(member_offset_in_bits(&typ, 2), 16);
    }

    #[test]
    fn member_offset_in_bits_struct2() {
        use super::member_offset_in_bits;

        let typ = ssa::Type::Structure(ssa::StructureType {
            members: vec![ssa::Type::I32.array_of(2), ssa::Type::I32],
        });

        assert_eq!(member_offset_in_bits(&typ, 0), 0);
        assert_eq!(member_offset_in_bits(&typ, 1), 16);
    }
}
