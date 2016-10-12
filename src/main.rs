extern crate unix_socket;

use std::io;
use std::env;
use std::mem;
use std::io::prelude::*;
use unix_socket::{UnixStream};

fn main() {
	println!("Primary Node Manager (v1.0)");

	let path = format!("{}/primary.sock", env::current_dir().unwrap().to_str().unwrap());
	println!("[Socket] {}", path);
	
	loop {
		std::io::stdout().write_all(b"$ ").unwrap();
		std::io::stdout().flush().unwrap();
		let mut eof = false;
		
		
		
		let mut cmd = String::new();
		
		while !eof {
			
			let mut cmdbuf = String::new();
			io::stdin().read_line(&mut cmdbuf).unwrap();
			
			cmdbuf = cmdbuf.trim_right().to_string();
			
			let mut bytes = unsafe{ cmdbuf.as_mut_vec() };
			let l = bytes.len();

			if l > 0 {

				if bytes[l-1] == b'\\' {
					//bytes[l-1] = b'\0';
					bytes[l-1] = b'\n';
					//std::io::stdout().write_all(b">>> ").unwrap();
					std::io::stdout().flush().unwrap();
				} else {
					eof = true;
				}
			} else {
				//std::io::stdout().write_all(b">>> ").unwrap();
				std::io::stdout().flush().unwrap();
				continue;
			}
			
			cmd.push_str(&String::from_utf8(bytes.clone()).unwrap());
		}
		if cmd == "exit" { break; }
		
		let len = cmd.len() as u32;
		let outlen : [u8;4] = unsafe { mem::transmute(len) };
		
		let mut stream = match UnixStream::connect(&path) {
			Err(e) => { println!("[Error] {}", e); continue; }
			Ok(s) => s,
		};
		
		match stream.write_all( &outlen ) {
			Err(e) => { println!("[Error] {}", e); continue; }
			_ => {}
		}

		match stream.write_all(cmd.as_bytes()) {
			Err(e) => { println!("[Error] {}", e); continue; }
			_ => {}
		}
	
		
	
		let mut response = String::new();
		stream.read_to_string(&mut response).unwrap();
		println!("{}", response);
	}
	
}