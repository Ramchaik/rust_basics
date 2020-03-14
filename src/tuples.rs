/* 
	Tuples group together values of different types
	Max 12 elements

*/
pub fn run() {
	let person: (&str, &str, i8) = ("Hello", "world", 11);

	println!("Message is {} to {}, {} times", person.0, person.1, person.2);
}