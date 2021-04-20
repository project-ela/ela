use super::{Type, Types, Value};

pub(crate) fn gep_elm_typ(types: &Types, val: &Value, indices: &[Value]) -> Type {
    let mut typ = val.typ();
    for _ in 0..indices.len() {
        typ = types.elm_typ(typ);
    }

    typ
}
