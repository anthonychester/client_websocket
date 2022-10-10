//! Home Page
#![warn(missing_docs)]

use std::io::Read;
use std::io::Write;
use std::net::{Shutdown, TcpStream};

#[derive(Debug)]
pub enum Opcode {
  Continue = 0x0,
  Text = 0x1,
  Binary = 0x2,
  Close = 0x8,
  Ping = 0x9,
  Pong = 0xa,
}

impl From<Opcode> for u8 {
  fn from(opcode: Opcode) -> Self {
  return match opcode {
    Opcode::Continue => 0x0,
    Opcode::Text => 0x1,
    Opcode::Binary => 0x2,
    Opcode::Close => 0x8,
    Opcode::Ping => 0x9,
    Opcode::Pong => 0xa,
    }
  }
}

pub fn connect(host: &'static str) {
  let mut stream = TcpStream::connect(host).expect("Couldn't connect to the server...");
  println!("CE");
  let mut http_req = String::from("");
  http_req.push_str("GET / HTTP/1.1");
  //http_req.push_str("\r\n");
  //http_req.push_str(format!("GET {} HTTP/1.1", host).as_str());
  http_req.push_str("\r\n");
  http_req.push_str(format!("Host: {}", host).as_str());
  http_req.push_str("\r\n");
  http_req.push_str("Connection: Upgrade");
  http_req.push_str("\r\n");
  http_req.push_str("Pragma: no-cache");
  http_req.push_str("\r\n");
  http_req.push_str("Cache-Control: no-cache");
  http_req.push_str("\r\n");
  http_req.push_str("Upgrade: websocket");
  http_req.push_str("\r\n");
  http_req.push_str("Sec-WebSocket-Version: 13");
  http_req.push_str("\r\n");
  http_req.push_str("Sec-WebSocket-Key: q4xkcO32u266gldTuKaSOw==");
  http_req.push_str("\r\n");

  //let response = "GET / HTTP/1.1\r\n\r\n";
  let response = "hi";
  println!("===> http_req = \n-----\n{}\n-----\n", response);
  let req_bytes: &[u8] = response.as_bytes();
  stream
    .write_all(req_bytes)
    .expect("Could not write");
  
  let mut http_res = String::new();
  stream
    .read_to_string(&mut http_res)
    .expect("Failed to read responce");
  println!("===> http_res = \n-----\n{}\n-----\n", http_res);
  
  stream
    .shutdown(Shutdown::Both)
    .expect("shutdown call failed");
  println!("Done");
}

fn handle_connection(mut stream: TcpStream) {
  stream.write(&[1]);
}

fn get_message(fin: bool) {
  let mut f: u8 = 0b0000;
  if fin {f = 0b1000}
  let op: u8 = Opcode::Pong as u8;
  println!("{:#b}", op);
}



/*

//cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

*/