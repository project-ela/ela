extern crate x86asm;

use x86asm::{
    decode, encode,
    instruction::{
        mnemonic::Mnemonic,
        operand::{
            immediate::Immediate,
            memory::{Displacement, Memory},
            offset::Offset,
            register::Register,
            Operand,
        },
        Instruction,
    },
};

#[test]
fn nullary() {
    do_test(Instruction::new_nullary(Mnemonic::Hlt));
    do_test(Instruction::new_nullary(Mnemonic::Ret));
    do_test(Instruction::new_nullary(Mnemonic::Syscall));
}

#[test]
fn unary_m() {
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Memory(Memory::new(Register::Rax, None)),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Memory(Memory::new(Register::R8, None)),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Memory(Memory::new(Register::Rax, Some(Displacement::Disp8(2)))),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Memory(Memory::new(Register::R8, Some(Displacement::Disp8(2)))),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Memory(Memory::new_disp(Displacement::Disp32(2))),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Memory(Memory::new(Register::R12, Some(Displacement::Disp8(0)))),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Memory(Memory::new(Register::R13, Some(Displacement::Disp8(0)))),
    ));

    do_test(Instruction::new_unary(
        Mnemonic::Sete,
        Operand::Register(Register::Al),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Sete,
        Operand::Register(Register::R8b),
    ));
}

#[test]
fn unary_o() {
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Register(Register::R8),
    ));
}

#[test]
fn unary_i() {
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_unary(
        Mnemonic::Push,
        Operand::Immediate(Immediate::Imm32(-2)),
    ));
}

#[test]
fn unary_d() {
    do_test(Instruction::new_unary(
        Mnemonic::Call,
        Operand::Offset(Offset::Off32(12)),
    ));
}

#[test]
fn binary_mi() {
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Eax),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::R8),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::Rax, None)),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R8, None)),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R12, Some(Displacement::Disp8(0)))),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R13, Some(Displacement::Disp8(0)))),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::Rax, Some(Displacement::Disp8(2)))),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R8, Some(Displacement::Disp8(2)))),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new_disp(Displacement::Disp32(2))),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
}

#[test]
fn binary_mr() {
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::R8),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Register(Register::R9),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::R8),
        Operand::Register(Register::R9),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::Rax, None)),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R8, None)),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R12, Some(Displacement::Disp8(0)))),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R13, Some(Displacement::Disp8(0)))),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::Rax, Some(Displacement::Disp8(2)))),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new(Register::R8, Some(Displacement::Disp8(2)))),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Memory(Memory::new_disp(Displacement::Disp32(2))),
        Operand::Register(Register::Rax),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Mov,
        Operand::Memory(Memory::new(Register::Rax, None)),
        Operand::Register(Register::R8b),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Mov,
        Operand::Memory(Memory::new(Register::R8, None)),
        Operand::Register(Register::Sil),
    ));
}

#[test]
fn binary_rm() {
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new(Register::Rax, None)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new(Register::R8, None)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new(Register::R12, Some(Displacement::Disp8(0)))),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new(Register::R13, Some(Displacement::Disp8(0)))),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new(Register::Rax, Some(Displacement::Disp8(2)))),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new(Register::R8, Some(Displacement::Disp8(2)))),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new_disp(Displacement::Disp32(2))),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::Add,
        Operand::Register(Register::Rax),
        Operand::Memory(Memory::new(Register::Rip, Some(Displacement::Disp32(0)))),
    ));
}

#[test]
fn binary_rmi() {
    do_test(Instruction::new_binary(
        Mnemonic::IMul,
        Operand::Register(Register::Eax),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::IMul,
        Operand::Register(Register::Rax),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::IMul,
        Operand::Register(Register::R8),
        Operand::Immediate(Immediate::Imm8(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::IMul,
        Operand::Register(Register::Eax),
        Operand::Immediate(Immediate::Imm32(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::IMul,
        Operand::Register(Register::Rax),
        Operand::Immediate(Immediate::Imm32(2)),
    ));
    do_test(Instruction::new_binary(
        Mnemonic::IMul,
        Operand::Register(Register::R8),
        Operand::Immediate(Immediate::Imm32(2)),
    ));
}

fn do_test(inst: Instruction) {
    let code = encode::encode(&inst);
    let inst_disasm = &decode::decode(&code)[0];

    assert_eq!(&inst, inst_disasm);
}
