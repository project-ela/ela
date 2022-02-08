use super::Function;

#[derive(Debug)]
pub struct Assembly {
    pub data: DataSection,
    pub text: TextSection,
}

#[derive(Debug)]
pub struct DataSection {
    pub items: Vec<DataSectionItem>,
}

#[derive(Debug)]
pub enum DataSectionItem {
    Data { name: String, bytes: Vec<DataItem> },
}

#[derive(Debug)]
pub enum DataItem {
    Zero(usize),
    Byte(i8),
    Long(i32),
}

#[derive(Debug)]
pub struct TextSection {
    pub functions: Vec<Function>,
}

impl Assembly {
    pub fn new() -> Self {
        Self {
            data: DataSection::new(),
            text: TextSection::new(),
        }
    }
}

impl DataSection {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_data(&mut self, name: String, bytes: Vec<DataItem>) {
        self.items.push(DataSectionItem::Data { name, bytes });
    }
}

impl TextSection {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
}

impl Assembly {
    pub fn stringify(&self) -> String {
        let mut s = String::new();

        s.push_str(".intel_syntax noprefix\n");

        s.push_str(&self.data.stringify());
        s.push_str(&self.text.stringify());

        s
    }
}

impl DataSection {
    pub fn stringify(&self) -> String {
        let mut s = String::new();

        s.push_str(".data\n");
        for item in &self.items {
            s.push_str(&item.stringify())
        }

        s
    }
}

impl DataSectionItem {
    pub fn stringify(&self) -> String {
        use self::DataSectionItem::*;

        let mut s = String::new();

        match self {
            Data { name, bytes } => {
                s.push_str(&format!("{}:\n", name));
                s.extend(bytes.iter().map(|item| item.stringify()));
            }
        }

        s
    }
}

impl DataItem {
    pub fn stringify(&self) -> String {
        use self::DataItem::*;

        match self {
            Zero(size) => format!(".zero {}\n", size),
            Byte(val) => format!(".byte {}\n", val),
            Long(val) => format!(".long {}\n", val),
        }
    }
}

impl TextSection {
    pub fn stringify(&self) -> String {
        let mut s = String::new();

        s.push_str(".text\n");
        for func in &self.functions {
            s.push_str(&func.stringify());
            s.push('\n');
        }

        s
    }
}
