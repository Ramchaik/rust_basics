 pub fn run() {
	let fixed_str = "f_string";
	let mut _str = String::from("d_string");
	
	// get length
	println!("Length: {}", _str.len());

	// push char
	_str.push('_');

	// push string
	_str.push_str(" hello.");

	// Capacity in bits
	println!("Capacity: {}", _str.capacity());

	// check if empty
	println!("is_empty() : {}", _str.is_empty());

	// Contains
	println!("Contains 'hello': {}", _str.contains("hello"));

	// Replace
	println!("Replace: {}", _str.replace("hello", "yellow"));
  
	// Loop through string using whitespace
	for word in _str.split_whitespace() {
		println!("{}", word);
	}

	// Create string with capacity
	let mut s = String::with_capacity(10);
	
	s.push('a');
	s.push('b');

	// Assertion testing
	assert_eq!(2, s.len());
	assert_eq!(10, s.capacity());

	println!("{}", s);
 }