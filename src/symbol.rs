use crate::*;

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ElfSymbol {
    pub name: ElfWord,
    pub info: u8,
    pub other: u8,
    pub section_index: ElfSection,
    pub value: ElfAddr,
    pub size: ElfXword,
}

pub enum Binding {
    Local = 0,
    Global = 1,
    Weak = 2,
}

pub enum Type {
    Notype = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    Common = 5,
    TLS = 6,
}

pub enum Visibility {
    Default = 0,
    Internal = 1,
    Hidden = 2,
    Protected = 3,
}

pub enum IndexType {
    Undef,
    Abs,
    Common,
    Index(u16),
}

#[allow(dead_code)]
pub const SYM_ENTRY_SIZE_32: ElfXword = 0x10;
#[allow(dead_code)]
pub const SYM_ENTRY_SIZE_64: ElfXword = 0x18;

impl ElfSymbol {
    pub fn set_binding(&mut self, binding: Binding) {
        self.info |= (binding as u8) << 4;
    }

    pub fn set_type(&mut self, typ: Type) {
        self.info |= typ as u8;
    }

    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.other |= visibility as u8;
    }

    pub fn set_value(&mut self, value: ElfAddr) {
        self.value = value;
    }

    pub fn set_index_type(&mut self, typ: IndexType) {
        self.section_index = match typ {
            IndexType::Undef => 0x0,
            IndexType::Abs => 0xfff1,
            IndexType::Common => 0xfff2,
            IndexType::Index(value) => value,
        };
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
