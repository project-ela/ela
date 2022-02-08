use super::{Type, Types, Value};

pub(crate) fn gep_elm_typ(types: &Types, val: &Value, indices: &[Value]) -> Type {
    let src_typ = val.typ();
    let mut elm_typ = types.elm_typ(src_typ);

    for i in 0..indices.len() {
        if i == 0 {
            continue;
        }

        match elm_typ {
            Type::Array(_, _) => elm_typ = types.elm_typ(elm_typ),
            x => unimplemented!("{:?}", x),
        }
    }

    elm_typ
}

pub(crate) fn gep_return_typ(types: &mut Types, val: &Value, indices: &[Value]) -> Type {
    let elm_typ = gep_elm_typ(types, val, indices);
    types.ptr_to(elm_typ)
}
