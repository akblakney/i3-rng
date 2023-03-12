use std::net::TcpStream;
use byteorder::{ByteOrder,LittleEndian};
use std::io::Write;
use std::io::Read;

const BUF_SIZE: usize = 64;

pub fn handle_query(stream: &mut TcpStream) {// -> Result<u8, String> {

  let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
  stream.write(b"query").unwrap();
  stream.read(&mut buf).unwrap();

  let entropy_est: u16 = LittleEndian::read_u16(&buf);
  println!("entropy estimate: {}", entropy_est);
  std::process::exit(0);

}

