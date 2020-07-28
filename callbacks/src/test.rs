use super::*;

#[test]
fn two_callbacks_primitive_argument() {
	let mut callbacks = Callbacks::new();
	callbacks += |a: &i32| assert_eq!(*a, 5);
	callbacks.notify_all(&5);
	callbacks += |a: &i32| assert_eq!(*a, 5);
	callbacks.notify_all(&5);
	callbacks.notify_all(&5);
}
#[test]
fn two_callbacks_reference_argument() {
	let mut callbacks = Callbacks::new();
	callbacks += |a: &Vec<i32>| assert_eq!(*a, vec![1, 2, 3]);

	let vector = vec![1, 2, 3];
	callbacks.notify_all(&vector);

	callbacks += |a: &Vec<i32>| assert_eq!(*a, vec![1, 2, 3]);
	callbacks.notify_all(&vector);
	callbacks.notify_all(&vector);
}