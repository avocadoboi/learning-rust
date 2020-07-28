use std::ops::AddAssign;

pub struct Callbacks<'a, _Arguments, _Return>
{
	callbacks: Vec<Box<dyn 'a + Fn(&_Arguments) -> _Return>>,
}
impl<'a, _Arguments, _Return> Callbacks<'a, _Arguments, _Return>
{
	pub fn new() -> Self {
		Self{
			callbacks: Vec::new(),
		}
	}
	pub fn notify_all(&self, arguments: &_Arguments)
	{
		for callback in &self.callbacks {
			callback(arguments);
		}
	}
}
impl<'a, _Arguments, _Return, T> AddAssign<T> for Callbacks<'a, _Arguments, _Return>
	where T: 'a + Fn(&_Arguments) -> _Return
{
	fn add_assign(&mut self, other: T) {
		self.callbacks.push(Box::new(other));
	}
}

#[cfg(test)]
mod test;
