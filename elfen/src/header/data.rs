use super::Data;

impl From<u8> for Data {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Data::DataNone,
            1 => Data::Data2LSB,
            2 => Data::Data2MSB,
            x => Data::Unknown(x),
        }
    }
}

impl Into<u8> for Data {
    fn into(self) -> u8 {
        match self {
            Data::DataNone => 0,
            Data::Data2LSB => 1,
            Data::Data2MSB => 2,
            Data::Unknown(x) => x,
        }
    }
}
