#[repr(C)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Tse {
    pub symbol_index: u64,
    pub offset: i64,
    pub size: u64,
    pub align: u64,
}

impl Tse {
    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.symbol_index.to_le_bytes());
        buf.extend(&self.offset.to_le_bytes());
        buf.extend(&self.size.to_le_bytes());
        buf.extend(&self.align.to_le_bytes());
    }
}
