use super::Register;

#[derive(Debug, Clone)]
pub struct Indirect {
    pub base: Register,
    pub disp: Displacement,
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
            disp: Displacement::Immediate(disp),
        }
    }

    pub fn new_label(base: Register, disp: String) -> Self {
        Self {
            base,
            disp: Displacement::Label(disp),
        }
    }

    pub fn stringify(&self) -> String {
        // TODO
        format!(
            "qword ptr [{}{}]",
            self.base.stringify(),
            self.disp.stringify()
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
