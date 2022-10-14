//! Home Page
#![warn(missing_docs)]

use std::io::Read;
use std::io::Write;
use std::net::{Shutdown, TcpStream};

mod base64;
use base64::base64_encode;
mod sha1;
use sha1::{pad_message,compute};
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

  let (sws_key, sws_acc) = generate_sws();

  let mut http_req = String::from("");
  http_req.push_str("GET / HTTP/1.1\r\n");

  http_req.push_str(format!("Host: {}\r\n", host).as_str());
  http_req.push_str("Connection: Upgrade\r\nPragma: no-cache\r\nCache-Control: no-cache\r\nUpgrade: websocket\r\nSec-WebSocket-Version: 13\r\n");
  http_req.push_str(format!("Sec-WebSocket-Key: {}", sws_key).as_str());//q4xkcO32u266gldTuKaSOw==");
  http_req.push_str("\r\n\r\n");

  println!("===> http_req = \n-----\n{}\n-----\n", http_req);
  let req_bytes: &[u8] = http_req.as_bytes();
  stream
    .write_all(req_bytes)
    .expect("Could not write");
  
  //let mut http_res = String::new();
  let mut message: Vec<u8> = vec!();
  let mut not_null = false;
  let mut res_val: bool = false;
  while !not_null {
    let mut buf = [0 as u8; 1];
    stream.read(&mut buf);
    //println!("buf: {:?}", buf[0]);
    message.push(buf[0]);
    if message.len() >= 4  && &message[message.len()-4..] == &[13, 10, 13, 10] {
      break;
    }
  }
  let http_res: String = message.into_iter().map(|x| x as char).collect();
  let parse: Vec<&str> = http_res.split("\r\n").collect();
  if parse.contains(&format!("Sec-WebSocket-Accept: {}", sws_acc).as_str()) { //add more cases for code 101 and HTTP
    println!("!!!pass!!!");
  } else {
    println!("{} sent the following message:\n-----", host);
    for l in parse {
      println!("{}", l);
    }
  }

  /*
  stream
    .read_to_string(&mut http_res)
    .expect("Failed to read responce");
  println!("===> http_res = \n-----\n{}\n-----\n", http_res);
  */

  stream
    .shutdown(Shutdown::Both)
    .expect("shutdown call failed");
  println!("Done");
}

fn get_message(fin: bool) {
  let mut f: u8 = 0b0000;
  if fin {f = 0b1000}
  let op: u8 = Opcode::Pong as u8;
  println!("{:#b}", op);
}

fn generate_sws() -> (String, String) {
  let guid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"; //Preset value https://www.rfc-editor.org/rfc/rfc6455#section-1.3
  let mut rng_val: Vec<u8> = vec!();
  for i in 0..16 {
    rng_val.push(0x00);
  }
  //let sws_key = base64_encode(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
  //  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
  let sws_key = String::from("dGhlIHNhbXBsZSBub25jZQ==");
  let mut raw_value = sws_key.clone();
  raw_value.push_str(guid);
  let text_u8 = raw_value.clone().into_bytes(); //Remove clone
  let padded_message = pad_message(&text_u8).unwrap();
  let sha1_hash: Vec<u8> = compute(padded_message).unwrap(); //pass
  let sws_acc = base64_encode(&sha1_hash);
  // s3pPLMBiTxaQ9kYGzzhZRbK+xOo=
  // MGJmZDI3OTg3ODQxY2VkNDkxMTkxZDQyYzBiZTkzN2Y4NjYzYzI5Ng==
  return (sws_key, sws_acc);
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