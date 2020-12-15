pub mod decode;
pub mod execute;
pub mod modrm;

use crate::{emulator::cpu::Register, instruction::modrm::RM};

type Memory = usize;

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    // 00 /r
    AddRm8R8(RM, Register),
    // 01 /r
    AddRm32R32(RM, Register),
    // 0F 84 cd
    Jz32(u32),
    // 0F 94
    SetE(RM),
    // 0F 95
    SetNE(RM),
    // 0F 9C
    SetL(RM),
    // 0F 9D
    SetGE(RM),
    // 0F 9E
    SetLE(RM),
    // 0F 9F
    SetG(RM),
    // 0F B6 /r
    MovzxR32Rm8(Register, RM),
    // 29 /r
    SubRm32R32(RM, Register),
    // 2B /r
    SubR32Rm32(Register, RM),
    // 31 /r
    XorR32Rm32(Register, RM),
    // 39 /r
    CmpRm32R32(RM, Register),
    // 50+rd
    PushR32(Register),
    // 58
    PopR32(Register),
    // 6A ib
    PushImm8(u32),
    // 6B /r ib
    IMulR32Rm32Imm8(Register, RM, u8),
    // 74 cb
    Jz8(u8),
    // 81 /5 id
    SubRm32Imm32(RM, u32),
    // 83 /0 ib
    AddRm32Imm32(RM, u32),
    // 83 /5 ib
    SubRm32Imm8(RM, u8),
    // 83 /6 ib0xc5c9799fL
    XorRm32Imm32(RM, u32),
    // 83 /7 ib
    CmpRm32Imm8(RM, u8),
    // 89 /r
    MovRm32R32(RM, Register),
    // 8B /r
    MovR32Rm32(Register, RM),
    // 8D /r
    LeaR32M(Register, Memory),
    // 8F /0
    PopRm32(RM),
    // 90
    Nop,
    // B8+ rd id
    MovR32Imm32(Register, u32),
    // C3
    Ret,
    // C7 /0 id
    MovRm32Imm32(RM, u32),
    // EB cb
    ShortJump(u8),
    // F7 /5
    IMulRm32(RM),
    // F7 /7
    IDivRm32(RM),
    // FF /6
    PushRm32(RM),
}

#[cfg(test)]
mod tests {
    use crate::emulator::cpu::Register::*;
    use crate::emulator::*;
    use crate::instruction::{modrm::RM, Opcode};

    fn new_emu(bin: &[u8]) -> Emulator {
        let mut emu = Emulator::new(0x7C00, 0x7C00);
        for (i, v) in bin.iter().enumerate() {
            emu.set_memory8(0x7C00 + i, *v);
        }
        return emu;
    }

    #[test]
    fn push_r32() {
        let mut emu = new_emu(&[0x50]);
        emu.set_register(EAX, 0x20);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x1);
        assert_eq!(opcode, Opcode::PushR32(EAX));
        emu.exec(opcode);
        assert_eq!(
            emu.get_memory32(emu.get_register(ESP) as usize),
            emu.get_register(EAX)
        );
    }

    #[test]
    fn pop_r32() {
        let mut emu = new_emu(&[0x58]);
        emu.push32(0xB);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x1);
        assert_eq!(opcode, Opcode::PopR32(EAX));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EAX), 0xB);
    }

    #[test]
    fn push_imm8() {
        let mut emu = new_emu(&[0x6A, 0x0A]);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x2);
        assert_eq!(opcode, Opcode::PushImm8(0xA));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(emu.get_register(ESP) as usize), 0xA);
    }

    #[test]
    fn mov_rm32_r32() {
        let mut emu = new_emu(&[0x89, 0xE5, 0x89, 0x45, 0xFC]);

        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x2);
        assert_eq!(opcode, Opcode::MovRm32R32(RM::Register(EBP), ESP));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EBP), emu.get_register(ESP));

        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x5);
        let addr = (emu.get_register(EBP) as i32 - 4 as i32) as usize;
        assert_eq!(opcode, Opcode::MovRm32R32(RM::Memory(addr), EAX));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(addr), emu.get_register(EAX));
    }

    #[test]
    fn mov_r32_rm32() {
        let mut emu = new_emu(&[0x8B, 0x45, 0xFC]);
        emu.set_register(EBP, 0x7C00);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x3);
        let addr = (emu.get_register(EBP) as i32 - 4 as i32) as usize;
        assert_eq!(opcode, Opcode::MovR32Rm32(EAX, RM::Memory(addr)));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EAX), emu.get_memory32(addr));
    }

    #[test]
    fn pop_rm32() {
        let mut emu = new_emu(&[0x8F, 0x45, 0x04]);
        emu.push32(0xB);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x3);
        let addr = (emu.get_register(EBP) as i32 + 4 as i32) as usize;
        assert_eq!(opcode, Opcode::PopRm32(RM::Memory(addr)));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(addr), 0xB);
    }

    #[test]
    fn nop() {
        let mut emu = new_emu(&[0x90]);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x1);
        assert_eq!(opcode, Opcode::Nop);
        emu.exec(opcode);
    }

    #[test]
    fn mov_r32_imm32() {
        let mut emu = new_emu(&[0xB8, 0x41, 0x00, 0x00, 0x00]);
        let opcode = emu.decode();
        assert_eq!(opcode, Opcode::MovR32Imm32(EAX, 0x41));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EAX), 0x41);
    }

    #[test]
    fn mov_rm32_imm32() {
        let mut emu = new_emu(&[0xC7, 0x45, 0xFC, 0x0A, 0x00, 0x00, 0x00]);
        emu.set_register(EBP, 0x7C00);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x7);
        let addr = (emu.get_register(EBP) as i32 - 4 as i32) as usize;
        assert_eq!(opcode, Opcode::MovRm32Imm32(RM::Memory(addr), 0xA));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(addr), 0xA);
    }

    #[test]
    fn short_jump() {
        let mut emu = new_emu(&[0xEB, 0x76]);
        let opcode = emu.decode();
        assert_eq!(opcode, Opcode::ShortJump(0x76 + 0x2));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x76 + 0x2);
    }

    #[test]
    fn push_rm32() {
        let mut emu = new_emu(&[0xFF, 0x75, 0x04]);
        emu.set_register(EBP, 0x7C00);
        let addr = (emu.get_register(EBP) as i32 + 4 as i32) as usize;
        emu.set_memory32(addr, 0xA);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x3);
        assert_eq!(opcode, Opcode::PushRm32(RM::Memory(addr)));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(emu.get_register(ESP) as usize), 0xA);
    }
}
