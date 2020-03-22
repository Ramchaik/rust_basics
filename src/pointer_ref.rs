// Reference Pointer - Point to a resource in memory

pub fn run() {
	// Primite Array
	let arr1 = [1, 3, 2];
	let arr2 = arr1;

	// with non primitives, you need to use (&) to point to the resource, assign directly will not hold the value
	
	// Vector (non primitive)
	let vec1 = vec![1, 3, 2];
	let vec2 = &vec1;


	println!("Values: {:?}", (&vec1, vec2));
}