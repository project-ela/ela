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
