pub const MAGIC: u32 = 1986883378;
pub const VERSION: u32 = 1;

pub struct Metadata {
    pub magic: u32,
    pub version: u32,
    pub code_offset: u32,
    pub data_offset: u32,
}

impl Metadata {
    pub fn new(code_offset: u32, data_offset: u32) -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            code_offset,
            data_offset,
        }
    }
}
