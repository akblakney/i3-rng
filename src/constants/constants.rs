#![allow(dead_code)]
pub const ADDR: &str = "127.0.0.1:1234";

pub const INSUFFICIENT_ENTROPY_HASH: [u8; 64] = {
  let mut x: [u8; 64] = [0u8; 64];
  x[32] = 1;
  x
};

pub const MIN_ENTROPY: u16 = 16;

pub const HASH_SIZE: usize = 32;
