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
    println!("invalid command, run with --help or help to see options");
    std::process::exit(1);
  }

  let command = String::from(&args[1]);
  let mut stream = TcpStream::connect(ADDR).unwrap();

  if command == "query" {
    query::handle_query(&mut stream);
  } else if command == "rand" {
    rand::handle_rand(&mut stream, &args);
  } else if command == "help" || command == "-h" || command == "--help" {
    help::handle_help();
  } else {
    println!("nothing to be done");
  }
  

}
