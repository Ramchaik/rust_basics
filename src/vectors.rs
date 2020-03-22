// vectors - Re-sizable vectors

// import mem from std
use std::mem;

pub fn run() {
	let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

	// Re-assign value
	numbers[2] = 20;

	// Add on to vector
	numbers.push(5);
	numbers.push(6);

	// Pop off last value
	numbers.pop();

	println!("Numbers: {:?}", numbers);

	// Get single value
	println!("Single Value: {}", numbers[0]);

	// Get vector length
	println!("Vector Length: {}", numbers.len());

	// vectors are stack allocated
	println!("Vector occupied {} bytes", mem::size_of_val(&numbers));

	// Get Slice
	let slice: &[i32] = &numbers[1..3];
	println!("Slice: {:?}", slice);

	// Loop through vector values
	for x in numbers.iter() {
		println!("Number: {}", x);
	}

	// Loop & mutate values (similar to .map in JS)
	for x in numbers.iter_mut() {
		*x *= 2;
	}

	println!("NUmbers Vec: {:?}", numbers);
}