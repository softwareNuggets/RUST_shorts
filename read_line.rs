use std::io::{self, Write};
use std::process::Command;

fn cls() {

	Command::new("cmd")
		.args(&["/c", "cls"])
		.status()
		.unwrap();
}

fn main() {

	cls();

	let mut first_name = String::new();

	print!("Enter First Name:");
	io::stdout().flush().unwrap();

	io::stdin().read_line(&mut first_name)
			.expect("Error in read_line");

	println!("First name = {}",first_name);
}
