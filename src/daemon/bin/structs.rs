use blake2::{Blake2s256};

pub struct HashObj {
  pub hasher: Blake2s256,
  pub entropy_est: u16,
}

