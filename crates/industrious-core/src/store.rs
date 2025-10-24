// Copyright (c) 2025 Industrious One LLC
// SPDX-License-Identifier: MIT

use std::{any::Any, cell::Cell};

use crate::Reducer;

pub struct Store<S, R> {
	state: Cell<S>,
	reducer: R,
}

impl<S, R> Store<S, R>
where
	S: Copy,
	R: Reducer<S>,
{
	pub fn new(initial_state: S, root_reducer: R) -> Self {
		Self {
			state: Cell::new(initial_state),
			reducer: root_reducer,
		}
	}

	pub fn state(&self) -> S {
		self.state.get()
	}

	pub fn dispatch(&self, action: &dyn Any) {
		self.state
			.set(self.reducer.reduce(self.state.get(), action));
	}
}
