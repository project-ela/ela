use crate::*;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rela {
    pub offset: ElfAddr,
    pub info: ElfXword,
    pub addend: ElfSxword,
}

pub enum Type {
    None = 0,
    Plt32 = 4,
}

impl Rela {
    pub fn set_info(&mut self, sym: u64, typ: Type) {
        self.info = (sym << 32) + (typ as u64)
    }

    pub fn set_addend(&mut self, addend: i64) {
        self.addend = addend;
    }

    pub fn get_symbol(&self) -> u64 {
        self.info >> 32
    }

    pub fn get_type(&self) -> u64 {
        self.info & 0xffffffff
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.offset.to_le_bytes());
        buf.extend(&self.info.to_le_bytes());
        buf.extend(&self.addend.to_le_bytes());
    }
}
