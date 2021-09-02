use super::Binding;

impl From<u8> for Binding {
    fn from(bytes: u8) -> Self {
        match bytes {
            0 => Binding::Local,
            1 => Binding::Global,
            2 => Binding::Weak,
            x => Binding::Unknown(x),
        }
    }
}

impl Into<u8> for Binding {
    fn into(self) -> u8 {
        match self {
            Binding::Local => 0,
            Binding::Global => 1,
            Binding::Weak => 2,
            Binding::Unknown(x) => x,
        }
    }
}
