use super::{Function, Module, Type, Value};

pub(crate) fn gep_elm_typ(
    module: &Module,
    function: &Function,
    val: &Value,
    indices: &[Value],
) -> Type {
    let mut typ = val.typ();
    for _ in 0..indices.len() {
        typ = match val {
            Value::Global(_) | Value::Parameter(_) => module.types.elm_typ(typ),
            _ => function.types.elm_typ(typ),
        };
    }

    typ
}
