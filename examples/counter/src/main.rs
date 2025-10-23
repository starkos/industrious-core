// Copyright (c) 2025 Industrious One, LLC
// SPDX-License-Identifier: MIT

use std::io::{self, Error, ErrorKind, Write};

// Let's start out by defining our program's state. For this example, all
// we're tracking is one integer value, but this state can become as complex
// as you like,

#[derive(Default)]
struct ProgramState {
	pub count: isize,
}

// For the moment, to get this test harness up and running, I'm hardcoding
// the available operations on the state. Once things are properly up and
// running we'll be doing this through mutation messages instead.

impl ProgramState {
	pub fn increment(&mut self, amount: isize) -> io::Result<()> {
		self.count += amount;
		Ok(())
	}

	pub fn reset(&mut self) -> io::Result<()> {
		self.count = 0;
		Ok(())
	}
}

// The program itself starts here with good ol' `main()`.

fn main() -> io::Result<()> {
	show_usage();

	// Initialize the program state
	let mut state = ProgramState::default();

	// Run the user input loop
	let mut stdout = io::stdout().lock();

	loop {
		// Display an input prompt for the user
		write!(stdout, "> ")?;
		stdout.flush()?;

		// Read the next command from the user
		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer)?;
		let input = buffer.trim();

		// Process the user's input. Right now I'm mutating the program's
		// state directly, which obviously isn't behavior which scales to
		// larger programs. Later this will be done by dispatch messages
		// to the store

		let (first_char, remainder) = input.split_at(1);
		let result = match first_char {
			"+" => match parse_amount(remainder) {
				Ok(amount) => state.increment(amount),
				Err(error) => Err(error),
			},

			"-" => match parse_amount(remainder) {
				Ok(amount) => state.increment(-amount),
				Err(error) => Err(error),
			},

			"r" => state.reset(),

			"q" => break,

			_ => Err(Error::new(
				ErrorKind::InvalidInput,
				format!("unknown command \"{}\"", input),
			)),
		};

		// Display the new result; when everything is up and running this
		// will be done in a store observer, and be event driven
		match result {
			Ok(_) => writeln!(stdout, "Counter value is now {}", state.count)?,
			Err(error) => writeln!(stdout, "{}", error)?,
		}
	}

	Ok(())
}

fn parse_amount(input: &str) -> io::Result<isize> {
	let input = input.trim();

	// if no number is provided, default to one
	if input.is_empty() {
		return Ok(1);
	}

	// otherwise try to parse and return the integer value
	input.parse::<isize>().map_err(|_| {
		Error::new(
			ErrorKind::InvalidInput,
			format!("\"{}\" is not an integer", input),
		)
	})
}

fn show_usage() {
	println!(
		r#"
Industrious Core Counter example

 + to increment
 - to decrement
 r to reset the count
 q to quit

Examples:
> +
> +4
> -1
	"#
	);
}
