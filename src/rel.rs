pub mod typ;

use crate::*;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rela {
    pub offset: ElfAddr,
    pub info: ElfXword,
    pub addend: ElfSxword,
}

#[derive(Eq, PartialEq)]
pub enum Type {
    None,
    Plt32,
    Unknown(u64),
}

impl Rela {
    pub fn set_info(&mut self, sym: u64, typ: Type) {
        let typ_bytes: u64 = typ.into();
        self.info = (sym << 32) + typ_bytes;
    }

    pub fn get_symbol(&self) -> u64 {
        self.info >> 32
    }

    pub fn get_type(&self) -> Type {
        let bytes = self.info & 0xffffffff;
        Type::from(bytes)
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.offset.to_le_bytes());
        buf.extend(&self.info.to_le_bytes());
        buf.extend(&self.addend.to_le_bytes());
    }
}
