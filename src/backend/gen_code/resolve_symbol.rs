use x86asm::instruction::operand::{offset::Offset, Operand};

use crate::backend::gen_code::{encode_item, Code, CodeItem, Codes, Rela, Symbol, Symbols};

pub fn resolve_symbol(symbols: &Symbols, code: &mut Code) -> Vec<Rela> {
    let mut relas = Vec::new();
    for unresolved_jump in &code.unresolved_jumps {
        let symbol = symbols.get(&unresolved_jump.label_name).unwrap();

        let offset = if symbol.is_global || symbol.addr.is_none() {
            0
        } else {
            let item_index = unresolved_jump.item_index + 1;
            let symbol_addr = symbol.addr.unwrap();
            calc_offset(&code.items, item_index, symbol_addr)
        };

        let item = code.items.get_mut(unresolved_jump.item_index).unwrap();
        match item {
            CodeItem::Inst(inst) => {
                inst.operand1 = Some(Operand::Offset(Offset::Off32(offset)));
            }
            _ => panic!(),
        }

        if symbol.is_global || symbol.addr.is_none() {
            let item_index = unresolved_jump.item_index + 1;
            let rela_offset = calc_offset(&code.items, 0, item_index) as u32 - 4;

            relas.push(Rela {
                name: symbol.name.to_string(),
                offset: rela_offset,
            });
        }
    }
    relas
}

pub fn list_global_symbols(symbols: Symbols, codes: &Codes) -> Vec<Symbol> {
    let mut global_symbols: Vec<Symbol> = symbols
        .into_iter()
        .map(|(_, v)| v)
        .filter(|symbol| symbol.is_global | symbol.addr.is_none())
        .map(|symbol| relocate_symbol(symbol, &codes))
        .collect();

    global_symbols.sort_by_key(|symbol| symbol.addr);

    global_symbols
}

fn relocate_symbol(mut symbol: Symbol, codes: &Codes) -> Symbol {
    let code = codes.get(&symbol.section).unwrap();

    symbol.addr = symbol
        .addr
        .map(|addr| calc_offset(&code.items, 0, addr) as usize);

    symbol
}

fn calc_offset(items: &[CodeItem], from: usize, to: usize) -> i32 {
    // make from <= to
    let sign = if from < to { 1 } else { -1 };
    let (from, to) = if from < to { (from, to) } else { (to, from) };

    items[from..to]
        .iter()
        .map(|item| encode_item(item).len() as i32)
        .sum::<i32>()
        * sign
}
