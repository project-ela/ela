use super::Register;

#[derive(Debug, Clone)]
pub struct Indirect {
    pub base: Register,
    pub disp: i32,
}

impl Indirect {
    pub fn new(base: Register, disp: i32) -> Self {
        Self { base, disp }
    }

    pub fn stringify(&self) -> String {
        // TODO
        match self.disp {
            0 => format!("qword ptr [{}]", self.base.stringify()),
            _ => format!("qword ptr [{}{:+}]", self.base.stringify(), self.disp),
        }
    }
}
