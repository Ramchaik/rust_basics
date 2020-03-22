// Loops - Used to iterate until a condition is met

pub fn run() {
	let mut count = 0;

	// Loop
	/* 
	loop {
		count += 1;
		println!("Number: {}", count);

		// condition to break the loop
		if count == 20 {
			break;
		}
	} */ 

	// While loop (FizzBuzz)
	/* 
	while count <= 100 {
		if count % 15 == 0 {
			println!("Number: {} FizzBuzz", count);
		} else if count % 3 == 0 {
			println!("Number: {} Fizz", count);
		} else if count % 5 == 0 {
			println!("Number: {} Buzz", count);
		} else {
			println!("Number: {} ", count);
		}
		count += 1;
	} */

	// For Range
	for x in 0..100 {
		if x % 15 == 0 {
			println!("Number: {} FizzBuzz", x);
		} else if x % 3 == 0 {
			println!("Number: {} Fizz", x);
		} else if x % 5 == 0 {
			println!("Number: {} Buzz", x);
		} else {
			println!("Number: {} ", x);
		}
	}
}