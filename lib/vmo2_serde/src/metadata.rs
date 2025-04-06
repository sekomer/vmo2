pub const MAGIC: u32 = 1986883378;

pub mod Version {
    pub const V1: u8 = 1;
    pub const V2: u8 = 2;
}

pub struct Metadata {
    pub magic: u32,
    pub version: u8,
    pub code_offset: u32,
    pub data_offset: u32,
}

impl Metadata {
    pub fn new(code_offset: u32, data_offset: u32) -> Self {
        Self {
            magic: MAGIC,
            version: Version::V1,
            code_offset,
            data_offset,
        }
    }
}
