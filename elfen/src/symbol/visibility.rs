use super::Visibility;

impl From<u8> for Visibility {
    fn from(bytes: u8) -> Self {
        match bytes {
            0 => Visibility::Default,
            1 => Visibility::Internal,
            2 => Visibility::Hidden,
            3 => Visibility::Protected,
            x => Visibility::Unknown(x),
        }
    }
}

impl Into<u8> for Visibility {
    fn into(self) -> u8 {
        match self {
            Visibility::Default => 0,
            Visibility::Internal => 1,
            Visibility::Hidden => 2,
            Visibility::Protected => 3,
            Visibility::Unknown(x) => x,
        }
    }
}
