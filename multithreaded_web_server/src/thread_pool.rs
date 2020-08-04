use std::thread;
use std::sync::{mpsc, Arc, Mutex};

struct Worker {
	thread: Option<thread::JoinHandle<()>>,
	id: usize,
}
impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
		Self{
			id,
			thread: Some(thread::spawn(move || loop {
				let message = receiver.lock().unwrap().recv().unwrap();
				match message {
					Message::NewJob(job) => {
						println!("Worker {} got job!", id);
						job();
						println!("Worker {} finished!", id);
					}
					Message::Terminate => {
						println!("Worker {} was told to terminate.", id);
						break;
					}
				}
			}))
		}
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
	NewJob(Job),
	Terminate,
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}
impl ThreadPool {
	pub fn new(thread_count: usize) -> Self {
		assert!(thread_count > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(thread_count);
		for id in 0..thread_count {
			println!("Created worker {}", id);
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}
		
		Self{workers, sender}
	}
	pub fn execute<F>(&self, function: F) -> Result<(), mpsc::SendError<Message>>
		where F: FnOnce() + Send + 'static
	{
		println!("Sending message to worker...");
		self.sender.send(Message::NewJob(Box::new(function)))?;
		println!("Sent!");
		Ok(())
	}
}
impl Drop for ThreadPool {
	fn drop(&mut self) {
		println!("Terminating all workers.");
		for _ in &self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}
		
		for worker in &mut self.workers {
			println!("Waiting for worker {} to finish.", worker.id);
			
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
		println!("All workers are now dead =)");
	}
}
