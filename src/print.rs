pub fn run() {
	// Print to console
	println!("Hello from print.rs file");

	// Basic formatting
	println!("{} is from {}", "Vaibhav", "Shimla");

	// Positional Arguments
	println!("{0} is from {1} and {0} like to {2}", "Vaibhav", "Shimla", "Code");

	// Named Arguments
	println!("{name} like to play {activity}", name = "Vaibhav", activity = "Hocus Pocus");

	// Placeholder traits
	println!("Bin: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
}

