use callbacks::Callbacks;

fn main() {
	let mut listeners = Callbacks::new();
	
	listeners += |(a, b, c): &(i32, i32, i32)| 
		println!("Listener 1 received message: ({}, {}, {})", a, b, c);
	
	listeners += |msg: &(i32, i32, i32)| 
		println!("Listener 2 received message: {:?}", msg);

	listeners += |(a, b, c): &(i32, i32, i32)| 
		println!("Listener 3 received message: ({}, {}, {})", a, b, c);
	
	listeners.notify_all(&(1, 2, 3));
}