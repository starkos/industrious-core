// Copyright (c) 2025 Industrious One LLC
// SPDX-License-Identifier: MIT

use std::{
	any::Any,
	io::{self, Write},
	num::ParseIntError,
	result,
};

use industrious_core::Store;

// Let's start out by defining our program's state, which for this simple
// example is just the counter value.

#[derive(Clone, Copy, Debug, Default)]
struct ProgramState {
	count: isize,
}

// Our list of actions. I'm using an `enum` here for convenience, but
// actions may be any type which implements `Any`.

enum Action {
	Increment { amount: isize },
	Decrement { amount: isize },
	Reset,
}

// The reducer accepts the current program state and an action, and
// performs the corresponding mutation(s). The operation is purely
// functional; the old state is left unchanged, and a new one is
// returned.

fn reducer(state: ProgramState, action: &dyn Any) -> ProgramState {
	if let Some(a) = action.downcast_ref::<Action>() {
		return match a {
			Action::Increment { amount } => ProgramState {
				count: state.count + amount,
			},

			Action::Decrement { amount } => ProgramState {
				count: state.count - amount,
			},

			Action::Reset => ProgramState { count: 0 },
		};
	}

	// If we don't recognize the action, return the state unmodified
	state
}

fn main() -> io::Result<()> {
	show_usage();

	// Initialize our program's state
	let store = Store::new(ProgramState::default(), reducer);

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

		// Parse the user's input, and send an appropriate action to the store
		let (first_char, remainder) = input.split_at(1);
		match first_char {
			"+" => match parse_amount(remainder) {
				Ok(amount) => store.dispatch(&Action::Increment { amount }),
				Err(_) => writeln!(stdout, "\"{}\" is not a valid integer", remainder)?,
			},

			"-" => match parse_amount(remainder) {
				Ok(amount) => store.dispatch(&Action::Decrement { amount }),
				Err(_) => writeln!(stdout, "\"{}\" is not a valid integer", remainder)?,
			},

			"r" => store.dispatch(&Action::Reset),

			"q" => break,

			_ => writeln!(stdout, "\"{}\" is not a valid command", input)?,
		};

		// TODO counter value should come out of an observer
		writeln!(stdout, "Counter value is now {}", store.state().count)?;
	}

	Ok(())
}

fn parse_amount(input: &str) -> result::Result<isize, ParseIntError> {
	let input = input.trim();

	// if no number is provided, default to one
	if input.is_empty() {
		return Ok(1);
	}

	// otherwise try to parse and return the integer value
	input.parse::<isize>()
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
