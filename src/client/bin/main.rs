use std::env;
use std::net::TcpStream;

#[path = "../../constants/constants.rs"]
mod constants;
mod query;
mod rand;
mod params;
mod help;
use crate::constants::{ADDR};


fn main() {

  // get command lines to send
  let args: Vec<String> = env::args().collect();

  // TODO add help and print to stderr
  if args.len() < 2 {
    eprintln!("invalid command, run with --help or help to see options");
    std::process::exit(1);
  }

  let command = String::from(&args[1]);
  if command == "help" || command == "-h" || command == "--help" {
    help::handle_help();
    std::process::exit(0);
  }

  let mut stream = TcpStream::connect(ADDR).unwrap();

  if command == "query" {
    query::handle_query(&mut stream);
  } else if command == "rand" {
    rand::handle_rand(&mut stream, &args);
  } else {
    eprintln!("invalid command, run 'i3-rng help' for help menu");
    std::process::exit(1);
  }
  

}
