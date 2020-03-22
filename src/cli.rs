use std::env;

pub fn run() {
	let args: Vec<String> = env::args().collect();
	let command = args[1].clone();
	let commands = &args[1..args.len()];
	let status = "100%";

	println!("Command: {:?}", command);
	println!("Commands: {:?}", commands);

	if command.to_lowercase() == "hello" {
		println!("Hi {}, how are you?", commands[1]);
	} else if command == "status" {
		println!("Status is {}", status);
	} else {
		println!("Not a valid command");
	}
}