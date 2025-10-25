// Copyright (c) 2025 Industrious One LLC
// SPDX-License-Identifier: MIT

use std::{any::Any, cell::RefCell};

use crate::Reducer;

pub struct Store<S, R> {
	state: RefCell<S>,
	reducer: R,
}

impl<S, R> Store<S, R>
where
	S: Copy,
	R: Reducer<S>,
{
	pub fn new(initial_state: S, root_reducer: R) -> Self {
		Self {
			state: RefCell::new(initial_state),
			reducer: root_reducer,
		}
	}

	pub fn dispatch(&self, action: &dyn Any) {
		self.state
			.replace_with(|state| self.reducer.reduce(state, action));
	}

	pub fn select<F, T>(&self, selector: F) -> T
	where
		F: Fn(&S) -> T,
	{
		selector(&self.state.borrow())
	}
}
