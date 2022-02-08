use x86asm::instruction::operand::register;

use super::value::Value;

pub struct Mmu {
    segments: Vec<Vec<u8>>,
    segment_details: Vec<SegmentDetail>,
}

struct SegmentDetail {
    virt_addr: usize,
    size: usize,
    index: usize,
}

pub struct PhysicalAddress {
    pub segment_index: usize,
    pub segment_offset: usize,
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            segment_details: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, virt_addr: usize, data: Vec<u8>) {
        let detail = SegmentDetail {
            virt_addr,
            size: data.len(),
            index: self.segments.len(),
        };

        self.segments.push(data);
        self.segment_details.push(detail);
    }

    pub fn calc_phys_addr(&self, virt_addr: usize) -> Result<PhysicalAddress, String> {
        for detail in &self.segment_details {
            let start_addr = detail.virt_addr;
            let end_addr = start_addr + detail.size;
            if virt_addr >= start_addr && virt_addr < end_addr {
                let offset = virt_addr - detail.virt_addr;
                return Ok(PhysicalAddress {
                    segment_index: detail.index,
                    segment_offset: offset,
                });
            }
        }

        Err("SEGV".to_string())
    }

    pub fn get_memory(&self, address: usize, size: &register::Size) -> Result<Value, String> {
        use register::Size::*;
        match size {
            Byte => self.get_memory8(address).map(|val| Value::Value8(val)),
            Word => unimplemented!(),
            DWord => self.get_memory32(address).map(|val| Value::Value32(val)),
            QWord => self.get_memory64(address).map(|val| Value::Value64(val)),
        }
    }

    pub fn set_memory(&mut self, address: usize, value: Value) -> Result<(), String> {
        match value {
            Value::Value8(value) => self.set_memory8(address, value),
            Value::Value32(value) => self.set_memory32(address, value),
            Value::Value64(value) => self.set_memory64(address, value),
        }
    }

    pub fn get_memory8(&self, address: usize) -> Result<u8, String> {
        let addr = self.calc_phys_addr(address)?;
        Ok(self.segments[addr.segment_index][addr.segment_offset])
    }

    pub fn set_memory8(&mut self, address: usize, value: u8) -> Result<(), String> {
        let addr = self.calc_phys_addr(address)?;
        self.segments[addr.segment_index][addr.segment_offset] = value;
        Ok(())
    }

    pub fn get_memory32(&self, address: usize) -> Result<u32, String> {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_memory8(address + i)? as u32) << (8 * i);
        }
        Ok(ret)
    }

    pub fn set_memory32(&mut self, address: usize, value: u32) -> Result<(), String> {
        for i in 0..4 {
            self.set_memory8(address + i, (value >> (8 * i)) as u8)?;
        }
        Ok(())
    }

    pub fn get_memory64(&self, address: usize) -> Result<u64, String> {
        let mut ret = 0;
        for i in 0..8 {
            ret |= (self.get_memory8(address + i)? as u64) << (8 * i);
        }
        Ok(ret)
    }

    pub fn set_memory64(&mut self, address: usize, value: u64) -> Result<(), String> {
        for i in 0..8 {
            self.set_memory8(address + i, (value >> (8 * i)) as u8)?;
        }
        Ok(())
    }
}
