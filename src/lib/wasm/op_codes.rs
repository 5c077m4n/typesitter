#![allow(dead_code)]

/// WASM magic: "\0asm"
pub const MAGIC: &[u8] = &[0x00, 0x61, 0x73, 0x6d];
/// WASM version: 1000
pub const VERSION: &[u8] = &[0x01, 0x00, 0x00, 0x00];

pub mod section {
    pub const TYPE: u8 = 0x01;
    pub const CODE: u8 = 0x0a;
    pub const FUNC: u8 = 0x03;
    pub const EXPORT: u8 = 0x07;
}

pub mod var_instr {
    pub const LOCAL_GET: u8 = 0x20;
}

pub mod i32_inst {
    pub const ADD: u8 = 0x6a;
}
pub mod f64_instr {
    pub const ADD: u8 = 0xa0;
    pub const SUB: u8 = 0xa1;
    pub const MUL: u8 = 0xa2;
    pub const DIV: u8 = 0xa3;
    pub const SQRT: u8 = 0x9f;
    pub const MIN: u8 = 0xa4;
    pub const MAX: u8 = 0xa5;
}

pub mod indices {
    pub const FUNC: u8 = 0x00;
}

pub mod control_flow {
    pub const FUNC: u8 = 0x60;
    pub const END: u8 = 0x0b;
}
