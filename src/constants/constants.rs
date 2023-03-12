// socket daemon and client use to communicate
#![allow(dead_code)]
pub const ADDR: &str = "127.0.0.1:1234";

// daemon sends this message to client when there is insufficient entropy
pub const INSUFFICIENT_ENTROPY_HASH: [u8; 64] = {
  let mut x: [u8; 64] = [0u8; 64];
  x[32] = 1;
  x
};

// minimum bits of entropy before we allow the client to get random bytes
pub const MIN_ENTROPY: u16 = 512;

// size of output of blake2s
pub const HASH_SIZE: usize = 32;
