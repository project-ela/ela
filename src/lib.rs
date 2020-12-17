pub mod common;
pub mod decode;
pub mod encode;
pub mod instruction;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
