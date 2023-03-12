//use generic_array::{GenericArray, ArrayLength};//, UInt, UTerm};
use std::thread;
use std::sync::{Arc, Mutex};
use blake2::{Blake2s256, Digest};

mod rng_server;
mod i3;
mod structs;
use crate::structs::{HashObj};

#[path = "../../constants/constants.rs"]
mod constants;

fn main(){

  println!("Daemon starting...");
  let m = Arc::new(Mutex::new( HashObj{
    hasher: Blake2s256::new(),
    entropy_est: 0u16}
   ));
  let m1 = Arc::clone(&m);
  let i3_thread = thread::spawn(move || i3::i3_listener(m1));
  let m2 = Arc::clone(&m);
  let rng_server_thread = thread::spawn(move || rng_server::rng_server_listen(m2));

  i3_thread.join().unwrap();
  rng_server_thread.join().unwrap();

}
