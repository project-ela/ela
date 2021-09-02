#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Strtab {
    pub data: Vec<u8>,
}

impl Strtab {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn insert(&mut self, s: String) -> usize {
        let name_index = self.data.len();

        self.data.extend(s.as_bytes());
        self.data.push(0);

        name_index
    }

    pub fn get(&self, index: usize) -> String {
        self.data[index..]
            .iter()
            .take_while(|&&v| v != 0)
            .map(|&v| v as char)
            .collect()
    }
}
