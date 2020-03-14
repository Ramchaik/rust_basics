pub fn run() {
	// default - i32
	let x = 31;

	// default - f64
	let y = 3.12;

	// Add explicit type
	let z: i64 = 109238091283;

	// Find max size
	println!("MAX size i32 {}", std::i32::MAX);
	println!("MAX size i64 {}", std::i64::MAX);

	// Boolean
	let is_active = true;
	// explicit way
	// let is_active: bool = true;

	// Get boolean from expresstion 
	let is_greater: bool = 10 < 5;

	// Char
	let a1 = 'a';
	let face = '\u{1F600}';

	println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

	  
}