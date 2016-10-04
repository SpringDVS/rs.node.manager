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
		let mut cmdbuf = String::new();
		io::stdin().read_line(&mut cmdbuf).unwrap();
		let cmd = cmdbuf.trim_right();
		
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