mod thread_pool;

use thread_pool::ThreadPool;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
	let listener = TcpListener::bind("127.0.0.1:7878")?;

	let thread_pool = ThreadPool::new(4);

	for stream in listener.incoming().take(2) {
		println!("New request!");
		let stream = stream?;
		thread_pool.execute(move || { 
			handle_connection(stream).unwrap();
		})?;
	}

	Ok(())
}

fn handle_connection(mut connection_stream: TcpStream) -> std::io::Result<()> {
	let mut request_buffer = [0; 1024];
	connection_stream.read(&mut request_buffer)?;
	connection_stream.write(get_response_from_request(&request_buffer)?.as_bytes())?;
	connection_stream.flush()?;

	Ok(())
}

fn get_response_from_request(request_buffer: &[u8]) -> std::io::Result<String> {
	let get_request_start = b"GET / HTTP/1.1\r\n";

	let (response_status_line, response_file_name) = if request_buffer.starts_with(get_request_start) {
		("HTTP/1.1 200 OK\r\n", "index.html")
	}
	else {
		("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
	};
	std::thread::sleep(std::time::Duration::from_secs(5));

	let website_root = String::from("website/");
	let response_file_contents = fs::read_to_string(website_root + response_file_name)?;
	
	Ok(format!(
		"{}Content-length: {}\r\n\r\n{}", 
		response_status_line, 
		response_file_contents.len(), 
		response_file_contents
	))
}
