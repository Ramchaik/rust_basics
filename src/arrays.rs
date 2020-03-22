// Arrays - fixed list where elements are same data type.

// import mem from std
use std::mem;

pub fn run() {
	let mut numbers: [i32; 4] = [1, 2, 3, 4];

	// Re-assign value
	numbers[2] = 20;

	println!("Numbers: {:?}", numbers);

	// Get single value
	println!("Single Value: {}", numbers[0]);

	// Get array length
	println!("Array Length: {}", numbers.len());

	// Arrays are stack allocated
	println!("Array occupied {} bytes", mem::size_of_val(&numbers));

	// Get Slice
	let slice: &[i32] = &numbers[1..3];
	println!("Slice: {:?}", slice);
}