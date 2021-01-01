use super::{Flags, SectionHeader, Type};

impl SectionHeader {
    pub fn get_type(&self) -> Type {
        Type::from(self.section_type)
    }

    pub fn set_type(&mut self, typ: Type) {
        self.section_type = typ.into();
    }

    pub fn set_flags(&mut self, flag: Flags) {
        let bytes: u64 = flag.into();
        self.flags |= bytes;
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.name.to_le_bytes());
        buf.extend(&self.section_type.to_le_bytes());
        buf.extend(&self.flags.to_le_bytes());
        buf.extend(&self.addr.to_le_bytes());
        buf.extend(&self.offset.to_le_bytes());
        buf.extend(&self.size.to_le_bytes());
        buf.extend(&self.link.to_le_bytes());
        buf.extend(&self.info.to_le_bytes());
        buf.extend(&self.alignment.to_le_bytes());
        buf.extend(&self.entry_size.to_le_bytes());
    }
}
