/* 
	Variables hold primitive data or refetences to data
	Varialbe are immutable by default
	Rust is block-scoped language
*/

pub fn run() {
	let name = "Vaibhav";
	let mut age = 23;

	age = 24;

	println!("My name is {name} and I am {age}", name = name, age = age);

	// Define constant
	const ID: i32 = 001;
	println!("ID: {}", ID);

	// Assign multiple vars
	let (my_name, my_age) = ("Vaibhav", 22);
	println!("{name} is {age}", name=my_name, age=my_age);

}