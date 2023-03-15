use std::str;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;//::{Write};//::{Read};
use std::sync::{Arc, Mutex};//mpsc;
use byteorder::{ByteOrder, LittleEndian};
use blake2::{Digest, Blake2s256};
use crate::constants::{ADDR, INSUFFICIENT_ENTROPY_HASH, MIN_ENTROPY, HASH_SIZE};
use crate::structs::{HashObj};

const BUF_SIZE: usize = 128;
const PAD1: u8 = 0;
const PAD2: u8 = 7;
const PADDED_HASH_SIZE: usize = HASH_SIZE + 8;

pub fn rng_server_listen(hash_mut: Arc<Mutex<HashObj>>) {

  let listener = TcpListener::bind(ADDR).unwrap();
  println!("listener is bound..");

  for stream in listener.incoming() {
    println!("found stream");
    match stream {
      Ok(mut stream) => {
        handle_stream(&mut stream, &hash_mut);
      },
      Err(e) => eprintln!("Error: {}", e)
    }
  } 
}

fn handle_stream(stream: &mut TcpStream, hash_mut: &Arc<Mutex<HashObj>>) {

  // vars
  let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];

  // try to read
  let read_result = stream.read(&mut buf);
  let num_bytes_read = match read_result {
    Ok(n) => n,
    Err(error) => {
      eprintln!("error reading from stream: {:?}", error);
      return;
    }
  };

  if num_bytes_read < 1 {
    return ;
  }

  // message from client
  let client_msg: &str = str::from_utf8(&buf[0..num_bytes_read]).unwrap();
  match client_msg {
    //"query" => eprintln!("{} bits of entropy", hash_obj.entropy_est),
    "query" => handle_query(hash_mut, stream),
    "rand" => handle_get_hash(hash_mut, stream),
    _ => handle_invalid_message(stream),
  }

}

fn handle_invalid_message(stream: &mut TcpStream) {
  match stream.write(b"Invalid message") {
    Err(e) => eprintln!("error writing to stream: {}", e),
    _ => (),
  }
}

fn handle_query(hash_mut: &Arc<Mutex<HashObj>>, stream: &mut TcpStream) {

  let hash_obj = hash_mut.lock().unwrap();
  let mut buf: [u8; 2] = [0u8; 2];
  LittleEndian::write_u16(&mut buf, hash_obj.entropy_est);
  
  match stream.write(&buf) {
    Err(e) => eprintln!("error writing to stream: {}", e),
    _ => (),
  }

}

// we send 64-byte message to client
// first 32 bytes are the hash value
// the next byte is a 0 if success, 1 if failure (i.e. insufficient entropy)
fn handle_get_hash(hash_mut: &Arc<Mutex<HashObj>>, stream: &mut TcpStream) {
  let mut hash_obj = hash_mut.lock().unwrap();

  // not enough entropy, warn client and exit
  if hash_obj.entropy_est < MIN_ENTROPY {
    match stream.write(&INSUFFICIENT_ENTROPY_HASH) {
      Err(e) => eprintln!("error writing to stream: {}",e),
      _ => (),
    }
    return;
  }

  let pool_out: [u8; HASH_SIZE] = hash_obj.hasher.finalize_reset().as_slice()
    .try_into().expect("wrong len of hash"); 

  // hash the output of the hash of the pool padded with 0s, then with 
  // nonzero (arbitrary) values, to get two separate, independent 256 bit output values
  let pre_hash1: [u8; PADDED_HASH_SIZE] = {
    let mut pre_hash1: [u8; PADDED_HASH_SIZE] = [PAD1; PADDED_HASH_SIZE];
    for i in 0..HASH_SIZE { 
      pre_hash1[i] = pool_out[i];
    }
    pre_hash1
  };
  let pre_hash2: [u8; PADDED_HASH_SIZE] = {
    let mut pre_hash2: [u8; PADDED_HASH_SIZE] = [PAD2; PADDED_HASH_SIZE];
    for i in 0..HASH_SIZE { 
      pre_hash2[i] = pool_out[i];
    }
    pre_hash2
  };

//  println!("prehash1 {:?}", pre_hash1);
//  println!("prehash2 {:?}", pre_hash2);

  let post_hash1: [u8; HASH_SIZE] = Blake2s256::digest(pre_hash1).as_slice()
    .try_into().expect("wrong length of hash");
  let post_hash2: [u8; HASH_SIZE] = Blake2s256::digest(pre_hash2).as_slice()
    .try_into().expect("wrong length of hash");

  hash_obj.hasher.update(post_hash1);
   
  let msg: [u8; 64] = {
    let mut msg: [u8; 64] = [0; 64];
    let suffix: [u8; HASH_SIZE] = [0u8; HASH_SIZE];
    let (one, two) = msg.split_at_mut(post_hash2.len());
    one.copy_from_slice(&post_hash2);
    two.copy_from_slice(&suffix);
    msg
  };

  match stream.write(&msg) {
    Err(e) => eprintln!("error writing to stream: {}",e),
    _ => (),
  }

}
