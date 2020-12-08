use crate::instruction::{Instruction, Mnemonic, Operand, RegSize, Register};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Generator {
    output: Vec<u8>,
    labels: HashMap<String, u32>,
    global_symbols: HashSet<String>,
    unknown_symbols: HashSet<String>,
    unresolved_jumps: Vec<UnresolvedJump>,
}

type UnresolvedJump = (String, u32);

pub struct GlobalSymbol {
    pub name: String,
    pub addr: u32,
}

pub struct GeneratedData {
    pub program: Vec<u8>,
    pub symbols: Vec<GlobalSymbol>,
    pub unknown_symbols: HashSet<String>,
}

pub fn generate(insts: Vec<Instruction>) -> Result<GeneratedData, String> {
    let mut generator = Generator::default();

    Ok(GeneratedData {
        program: generator.generate(insts)?,
        symbols: generator.global_symbols(),
        unknown_symbols: generator.unknown_symbols,
    })
}

impl Generator {
    fn generate(&mut self, insts: Vec<Instruction>) -> Result<Vec<u8>, String> {
        for inst in insts {
            self.gen_inst(inst)?;
        }
        self.resolve_jump()?;
        Ok(self.output.clone())
    }

    fn global_symbols(&self) -> Vec<GlobalSymbol> {
        let mut syms = Vec::new();

        for sym_name in &self.global_symbols {
            if let Some(addr) = self.labels.get(sym_name) {
                syms.push(GlobalSymbol {
                    name: sym_name.clone(),
                    addr: *addr,
                });
            }
        }

        syms
    }

    fn gen_inst(&mut self, inst: Instruction) -> Result<(), String> {
        match inst {
            Instruction::PseudoOp { name, arg } => match name.as_str() {
                ".global" => {
                    self.global_symbols.insert(arg);
                }
                _ => {}
            },
            Instruction::Label { name } => {
                let addr = self.output.len();
                self.labels.insert(name, addr as u32);
            }
            Instruction::NullaryOp(op) => self.gen_nullary_op(op)?,
            Instruction::UnaryOp(op, operand) => self.gen_unary_op(op, operand)?,
            Instruction::BinaryOp(op, operand1, operand2) => {
                self.gen_biary_op(op, operand1, operand2)?
            }
        }
        Ok(())
    }

    fn gen_nullary_op(&mut self, m: Mnemonic) -> Result<(), String> {
        match m {
            Mnemonic::Ret => self.gen(0xC3),
            x => return Err(format!("unexpected mnemonic: {:?}", x)),
        }
        Ok(())
    }

    fn gen_unary_op(&mut self, m: Mnemonic, operand: Operand) -> Result<(), String> {
        match m {
            Mnemonic::Push => match operand {
                Operand::Immidiate { value } => self.gen_i(0x6A, value),
                Operand::Register { reg } => {
                    if reg.size() != RegSize::QWord {
                        return Err(format!("unexpected operand: {:?}", reg));
                    }
                    self.gen_o(0x50, reg)
                }
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            Mnemonic::Pop => match operand {
                Operand::Register { reg } => {
                    if reg.size() != RegSize::QWord {
                        return Err(format!("unexpected operand: {:?}", reg));
                    }
                    self.gen_o(0x58, reg)
                }
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            Mnemonic::IDiv => match operand {
                Operand::Register { reg } => self.gen_m(&[0xF7], 7, reg),
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            Mnemonic::Jmp => self.gen_jump(&[0xE9], operand)?,
            Mnemonic::Sete => self.gen_set(0x94, operand)?,
            Mnemonic::Je => self.gen_jump(&[0x0F, 0x84], operand)?,
            Mnemonic::Setne => self.gen_set(0x95, operand)?,
            Mnemonic::Setl => self.gen_set(0x9C, operand)?,
            Mnemonic::Setle => self.gen_set(0x9E, operand)?,
            Mnemonic::Setg => self.gen_set(0x9F, operand)?,
            Mnemonic::Setge => self.gen_set(0x9D, operand)?,
            Mnemonic::Call => self.gen_jump(&[0xE8], operand)?,
            x => return Err(format!("unexpected mnemonic: {:?}", x)),
        }
        Ok(())
    }

    fn gen_biary_op(
        &mut self,
        m: Mnemonic,
        operand1: Operand,
        operand2: Operand,
    ) -> Result<(), String> {
        macro_rules! gen {
            ($op1: expr, $op2: expr, $reg1: expr, $op3: expr, $op4: expr) => {{
                match (&operand1, &operand2) {
                    (Operand::Register { .. }, Operand::Register { .. }) => {
                        self.gen_mr($op1, operand1, operand2)?
                    }
                    (Operand::Register { reg: reg1 }, Operand::Immidiate { value: value2 }) => {
                        self.gen_mi($op2, $reg1, reg1.clone(), *value2)
                    }
                    (Operand::Register { .. }, Operand::Address(_)) => {
                        self.gen_rm($op3, operand1, operand2)?
                    }
                    (Operand::Address(_), Operand::Register { .. }) => {
                        self.gen_mr($op4, operand1, operand2)?
                    }
                    _ => unimplemented!(),
                }
            }};
        }

        if !is_same_reg_size(&operand1, &operand2) {
            return Err(format!(
                "operand type mismatch: {:?} and {:?}",
                operand1, operand2
            ));
        }

        match m {
            Mnemonic::Mov => gen!(&[0x89], 0xC7, 0, &[0x8B], &[0x89]),
            Mnemonic::Add => gen!(&[0x01], 0x81, 0, &[0x03], &[0x01]),
            Mnemonic::Or => gen!(&[0x09], 0x81, 1, &[0x0b], &[0x09]),
            Mnemonic::And => gen!(&[0x21], 0x81, 4, &[0x23], &[0x21]),
            Mnemonic::Sub => gen!(&[0x29], 0x81, 5, &[0x2b], &[0x29]),
            Mnemonic::Xor => gen!(&[0x31], 0x81, 6, &[0x33], &[0x31]),
            Mnemonic::Cmp => gen!(&[0x39], 0x81, 7, &[0x3b], &[0x39]),

            Mnemonic::IMul => {
                let reg1 = expect_register(operand1)?;
                let reg2 = expect_register(operand2)?;
                self.gen_rm(
                    &[0x0F, 0xAF],
                    Operand::Register { reg: reg1 },
                    Operand::Register { reg: reg2 },
                )?;
            }

            x => return Err(format!("unexpected mnemonic: {:?}", x)),
        }

        Ok(())
    }

    // [opcodes]+[opr](4)
    fn gen_jump(&mut self, opcodes: &[u8], operand: Operand) -> Result<(), String> {
        match operand {
            Operand::Label { name } => {
                // jmp命令のオペランド部分の開始アドレス
                let jump_opr = (self.output.len() + opcodes.len()) as u32;
                // labelのアドレス(labelの次の命令の開始アドレス)
                let label_addr = self.lookup_label(name, jump_opr);
                // jmp命令の次の命令の開始アドレス
                let after_jump_addr = jump_opr + 4;
                let diff = label_addr.wrapping_sub(after_jump_addr);
                self.gen_d32(opcodes, diff);
            }
            x => return Err(format!("unexpected operand: {:?}", x)),
        }
        Ok(())
    }

    fn gen_set(&mut self, opcode: u8, operand: Operand) -> Result<(), String> {
        let reg1 = expect_register(operand)?;
        if reg1.size() != RegSize::Byte {
            return Err("expected r8".to_string());
        }
        self.gen_m(&[0x0F, opcode], 0, reg1);
        Ok(())
    }

    fn gen_i(&mut self, opcode: u8, imm: u32) {
        self.gen(opcode);
        self.gen(imm as u8);
    }

    fn gen_o(&mut self, opcode: u8, reg: Register) {
        if reg.only_in_64bit() {
            self.gen(calc_rex(false, false, false, true));
        }
        self.gen(opcode + reg.number());
    }

    fn gen_m(&mut self, opcodes: &[u8], reg: u8, r: Register) {
        if r.size() == RegSize::QWord || r.only_in_64bit() {
            self.gen(calc_rex(
                r.size() == RegSize::QWord,
                false,
                false,
                r.only_in_64bit(),
            ));
        }
        self.gen_bytes(opcodes);
        self.gen(calc_modrm(0b11, reg, r.number()));
    }

    fn gen_d32(&mut self, opcodes: &[u8], offset: u32) {
        self.gen_bytes(opcodes);
        self.gen32(offset);
    }

    fn gen_mr(&mut self, opcodes: &[u8], opr1: Operand, opr2: Operand) -> Result<(), String> {
        self.gen_rex(&opr2, &opr1);
        self.gen_bytes(opcodes);
        self.gen_modrm(opr2, opr1)
    }

    fn gen_mi(&mut self, opcode: u8, reg: u8, opr1: Register, opr2: u32) {
        if opr1.size() == RegSize::QWord {
            self.gen(calc_rex(true, false, false, opr1.only_in_64bit()));
        }
        self.gen(opcode);
        self.gen(calc_modrm(0b11, reg, opr1.number()));
        self.gen32(opr2);
    }

    fn gen_rm(&mut self, opcodes: &[u8], opr1: Operand, opr2: Operand) -> Result<(), String> {
        self.gen_rex(&opr1, &opr2);
        self.gen_bytes(opcodes);
        self.gen_modrm(opr1, opr2)
    }

    fn gen_rex(&mut self, opr1: &Operand, opr2: &Operand) {
        let reg1 = match opr1.clone() {
            Operand::Register { reg } => reg,
            Operand::Address(addr) => addr.base,
            _ => return,
        };

        let reg2 = match opr2.clone() {
            Operand::Register { reg } => reg,
            Operand::Address(addr) => addr.base,
            _ => return,
        };

        if reg1.size() != RegSize::QWord {
            return;
        }

        self.gen(calc_rex(
            true,
            reg1.only_in_64bit(),
            false,
            reg2.only_in_64bit(),
        ));
    }

    fn gen_modrm(&mut self, opr1: Operand, opr2: Operand) -> Result<(), String> {
        match opr1 {
            Operand::Register { reg: reg1 } => match opr2 {
                Operand::Register { reg: reg2 } => {
                    self.gen(calc_modrm(0b11, reg1.number(), reg2.number()))
                }
                Operand::Address(addr2) => match addr2.disp {
                    Some(disp) => {
                        self.gen(calc_modrm(0b01, reg1.number(), addr2.base.number()));
                        self.gen(disp as u8)
                    }
                    None => self.gen(calc_modrm(0b00, reg1.number(), addr2.base.number())),
                },
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            Operand::Address(addr1) => match opr2 {
                Operand::Register { reg: reg2 } => match addr1.disp {
                    Some(disp) => {
                        self.gen(calc_modrm(0b01, reg2.number(), addr1.base.number()));
                        self.gen(disp as u8)
                    }
                    None => self.gen(calc_modrm(0b00, reg2.number(), addr1.base.number())),
                },
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            x => return Err(format!("unexpected operand: {:?}", x)),
        }

        Ok(())
    }

    fn lookup_label(&mut self, name: String, code_addr: u32) -> u32 {
        match self.labels.get(&name) {
            Some(addr) => *addr,
            None => {
                self.unresolved_jumps.push((name, code_addr));
                0
            }
        }
    }

    fn resolve_jump(&mut self) -> Result<(), String> {
        for (name, jump_opr) in &self.unresolved_jumps {
            match self.labels.get(name) {
                Some(label_addr) => {
                    let after_jump_addr = jump_opr + 4;
                    let diff = (*label_addr).wrapping_sub(after_jump_addr);
                    for (i, byte) in diff.to_le_bytes().iter().enumerate() {
                        self.output[*jump_opr as usize + i] = *byte;
                    }
                }
                None => {
                    self.unknown_symbols.insert(name.clone());
                }
            }
        }
        Ok(())
    }

    fn gen32(&mut self, bytes: u32) {
        for i in 0..4 {
            let byte = (bytes >> (8 * i)) as u8;
            self.gen(byte);
        }
    }

    fn gen_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.gen(*byte)
        }
    }

    fn gen(&mut self, byte: u8) {
        self.output.push(byte);
    }
}

fn calc_rex(w: bool, r: bool, x: bool, b: bool) -> u8 {
    0b01000000 | (w as u8) << 3 | (r as u8) << 2 | (x as u8) << 1 | (b as u8)
}

fn calc_modrm(modval: u8, reg: u8, rm: u8) -> u8 {
    modval << 6 | reg << 3 | rm
}

fn expect_register(operand: Operand) -> Result<Register, String> {
    match operand {
        Operand::Register { reg } => Ok(reg),
        x => Err(format!("unexpected operand: {:?}", x)),
    }
}

fn is_same_reg_size(op1: &Operand, op2: &Operand) -> bool {
    match (op1, op2) {
        (Operand::Register { reg: reg1 }, Operand::Register { reg: reg2 }) => {
            reg1.size() == reg2.size()
        }
        _ => true,
    }
}
