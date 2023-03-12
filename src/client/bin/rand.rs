use std::net::TcpStream;
use std::io::Write;
use std::io::Read;
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

#[path = "../../constants/constants.rs"]
mod constants;
use crate::constants::{HASH_SIZE};

use crate::params;

const BUF_SIZE: usize = 64;
const DEFAULT_NUM_BYTES: usize = 64;
const DEFAULT_FORMAT: &str = "bytes";

const ALPHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "~!@#$%^&*()-_=+[{]}|;:'\",<.>/?";


pub fn handle_rand(stream: &mut TcpStream, args: &Vec<String>) {

  // handle num_bytes parameter
  let num_bytes = params::get_param(&args, "-n");
  let num_bytes = match num_bytes {
    Ok(x) => x.parse::<usize>().unwrap_or_else(|_| {
      eprintln!("must give integer argument to -n flag");
      std::process::exit(1);
      }),
    Err(_) => DEFAULT_NUM_BYTES,
  };

  // handle format parameter
  let format = params::get_param(&args, "-f");
  let format = match format {
    Ok(x) => x,
    Err(_) => DEFAULT_FORMAT,
  };


  //let rand_out = rand_from_daemon(stream, args[2].parse::<usize>().unwrap());
  let rand_out = rand_from_daemon(stream, num_bytes);
  let rand_out = match rand_out {
    Ok(rand_out) => rand_out,
    Err(e) => {
      eprintln!("error: {}", e);
      std::process::exit(1);
    }
  };

  let mut buffer: String = "".to_owned();
  match format {
    "bytes" => std::io::stdout().write_all(&rand_out).unwrap(),
    "hex" => println!("{}",hex::encode(&rand_out)),
    "digits" => {
      rand_from_charset(&DIGITS, rand_out, &mut buffer);
      println!("{}",buffer);
    },
    "alpha-numeric" => {
      let charset = format!("{}{}", ALPHA_LOWER, DIGITS);
      rand_from_charset(&charset, rand_out, &mut buffer);
      println!("{}",buffer);
    },
    "Alpha-numeric" => {
      let charset = format!("{}{}{}", ALPHA_LOWER, DIGITS, ALPHA_UPPER);
      rand_from_charset(&charset, rand_out, &mut buffer);
      println!("{}",buffer);
    },
    "ascii" => {
      let charset = format!("{}{}{}{}", ALPHA_LOWER, DIGITS, ALPHA_UPPER, SYMBOLS);
      rand_from_charset(&charset, rand_out, &mut buffer);
      println!("{}",buffer);
    },
    _ => {
    eprintln!("invalid format");
    std::process::exit(1);
    }
  }
}

// returns a Result with a random value in [a, b) if success
fn unbiased_u8(a: u8, b: u8, r: u8) -> Option<u8> {

  if b <= a {
    eprintln!("Error: invalid range");
    std::process::exit(1);
  }

  let range = b - a;
  let divisor: u8 = ((!range) / range) + 1;
  if divisor == 0 { //overflow is 2**8
    return Some(0);
  }
  let val: u8 = r / divisor;
  if val < range {
    return Some(val + a);
  }
  None
}

fn rand_from_charset<'a>(charset: &'a str, rand: Vec<u8>, buffer: &'a mut String) {

  let n: u8 = charset.len().try_into().expect("invalid conversion");
  for i in 0..rand.len() {
    let r = unbiased_u8(0, n, rand[i]);
    match r {
      Some(x) => {
        let index: usize = usize::try_from(x).unwrap(); 
        buffer.push_str(&charset[index..index+1]);
      },
      None => (),
    }
  }

}
  

fn rand_from_daemon(stream: &mut TcpStream, n: usize) -> Result<Vec<u8>, String> {

  // get key from daemon
  let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
  stream.write(b"rand").unwrap();
  stream.read(&mut buf).unwrap();

  // insufficient entropy
  if buf[HASH_SIZE] != 0 {
    return Err("no hash from server due to insufficient entropy".to_string());
  }

  let key: [u8; HASH_SIZE] = {
    let mut key = [0u8; HASH_SIZE];
    for i in 0..HASH_SIZE { key[i] = buf[i]; }
    key
  };
  let nonce = [0; 12];
  let mut plaintext: Vec<u8> = vec![0; n];
  let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
  cipher.apply_keystream(&mut plaintext);
  return Ok(plaintext);
}

