#[allow(non_snake_case)]
pub mod OPCODE {
    pub const HALT: u8 = 0;
    pub const LITERAL: u8 = 1;
    pub const ARITHMETIC: u8 = 2;
    pub const LOGIC: u8 = 3;
    pub const COMPARISON: u8 = 4;
    pub const MEMORY: u8 = 5;
    pub const IO: u8 = 6;

    pub const LITERAL_UINT: u8 = 0;
    pub const LITERAL_BOOL: u8 = 1;
    pub const LITERAL_STRING: u8 = 2;

    pub const ARITHMETIC_ADD: u8 = 0;
    pub const ARITHMETIC_SUB: u8 = 1;
    pub const ARITHMETIC_MUL: u8 = 2;
    pub const ARITHMETIC_DIV: u8 = 3;

    pub const LOGIC_AND: u8 = 0;
    pub const LOGIC_OR: u8 = 1;
    pub const LOGIC_XOR: u8 = 2;
    pub const LOGIC_NOT: u8 = 3;

    pub const COMPARISON_EQ: u8 = 0;
    pub const COMPARISON_NE: u8 = 1;
    pub const COMPARISON_LT: u8 = 2;
    pub const COMPARISON_LE: u8 = 3;
    pub const COMPARISON_GT: u8 = 4;
    pub const COMPARISON_GE: u8 = 5;

    pub const MEMORY_LOAD: u8 = 0;
    pub const MEMORY_STORE: u8 = 1;

    pub const IO_PRINT: u8 = 0;
    pub const IO_SCAN: u8 = 1;
}
