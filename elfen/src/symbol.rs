pub mod binding;
pub mod index_typ;
pub mod typ;
pub mod visibility;

use crate::*;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: ElfWord,
    pub info: u8,
    pub other: u8,
    pub section_index: ElfSection,
    pub value: ElfAddr,
    pub size: ElfXword,
}

#[derive(Eq, PartialEq)]
pub enum Binding {
    Local,
    Global,
    Weak,
    Unknown(u8),
}

#[derive(Eq, PartialEq)]
pub enum IndexType {
    Undef,
    Abs,
    Common,
    Index(u16),
}

#[derive(Eq, PartialEq)]
pub enum Type {
    Notype,
    Object,
    Func,
    Section,
    File,
    Common,
    TLS,
    Unknown(u8),
}

#[derive(Eq, PartialEq)]
pub enum Visibility {
    Default,
    Internal,
    Hidden,
    Protected,
    Unknown(u8),
}

impl Symbol {
    pub fn get_binding(&self) -> Binding {
        Binding::from(self.info >> 4)
    }

    pub fn set_binding(&mut self, binding: Binding) {
        let byte: u8 = binding.into();
        self.info |= byte << 4;
    }

    pub fn get_type(&self) -> Type {
        Type::from(self.info & 0xf)
    }

    pub fn set_type(&mut self, typ: Type) {
        let byte: u8 = typ.into();
        self.info |= byte;
    }

    pub fn get_visibility(&self) -> Visibility {
        Visibility::from(self.other)
    }

    pub fn set_visibility(&mut self, visibility: Visibility) {
        let byte: u8 = visibility.into();
        self.other |= byte;
    }

    pub fn get_index_type(&self) -> IndexType {
        IndexType::from(self.section_index)
    }

    pub fn set_index_type(&mut self, typ: IndexType) {
        self.section_index = typ.into();
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.name.to_le_bytes());
        buf.extend(&self.info.to_le_bytes());
        buf.extend(&self.other.to_le_bytes());
        buf.extend(&self.section_index.to_le_bytes());
        buf.extend(&self.value.to_le_bytes());
        buf.extend(&self.size.to_le_bytes());
    }
}
