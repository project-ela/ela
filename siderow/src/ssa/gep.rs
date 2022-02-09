use super::{Type, Value};

pub(crate) fn gep_elm_typ(val: &Value, indices: &[Value]) -> Type {
    let src_typ = val.typ();
    let mut elm_typ = src_typ.elm_typ();

    for i in 0..indices.len() {
        if i == 0 {
            continue;
        }

        match elm_typ {
            Type::Array(_, _) => elm_typ = elm_typ.elm_typ(),
            Type::Structure(s) => {
                let member_index = indices[i].as_i32();
                elm_typ = s.members[member_index as usize].clone();
            }
            x => unimplemented!("{:?}", x),
        }
    }

    elm_typ
}

pub(crate) fn gep_return_typ(val: &Value, indices: &[Value]) -> Type {
    let elm_typ = gep_elm_typ(val, indices);
    elm_typ.ptr_to()
}
