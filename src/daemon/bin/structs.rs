use blake2::{Blake2s256};//, Digest};

pub struct HashObj {
  pub hasher: Blake2s256,
  pub entropy_est: u16,
}

// pool is a vector containing the entropy pool
// entropy_est keeps track of the estimate of number of bits of entropy.
// Beyond a certain point there is no need to keep track...
//pub struct EntropyPool {
//  pub state: Vec<u8>,
//  pub entropy_est: u16,
//}

