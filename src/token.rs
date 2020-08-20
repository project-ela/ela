#[derive(Debug)]
pub enum Token {
    Integer { value: u32 },
    Ident { name: String },

    Commna,
    Colon,

    EOF,
}
