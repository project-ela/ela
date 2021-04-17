use super::Register;

#[derive(Debug, Clone)]
pub struct Indirect {
    pub base: Register,
    pub index: Option<Register>,
    pub disp_base: Displacement,
    pub disp_offset: i32,
}

#[derive(Debug, Clone)]
pub enum Displacement {
    Immediate(i32),
    Label(String),
}

impl Indirect {
    pub fn new_imm(base: Register, disp: i32) -> Self {
        Self {
            base,
            index: None,
            disp_base: Displacement::Immediate(disp),
            disp_offset: 0,
        }
    }

    pub fn new_label(base: Register, disp: String) -> Self {
        Self {
            base,
            index: None,
            disp_base: Displacement::Label(disp),
            disp_offset: 0,
        }
    }

    pub fn set_index(&mut self, index: Register) {
        self.index = Some(index);
    }

    pub fn set_disp_offset(&mut self, offset: i32) {
        self.disp_offset = offset;
    }

    pub fn stringify(&self) -> String {
        // TODO

        let index_str = match &self.index {
            Some(index) => format!("+{}*8", index.stringify()),
            None => "".into(),
        };

        format!(
            "qword ptr [{}{}{}{:+}]",
            self.base.stringify(),
            index_str,
            self.disp_base.stringify(),
            self.disp_offset,
        )
    }
}

impl Displacement {
    pub fn stringify(&self) -> String {
        use self::Displacement::*;

        match self {
            Immediate(0) => "".into(),
            Immediate(imm) => format!("{:+}", imm),
            Label(name) => format!("+{}", name),
        }
    }
}
