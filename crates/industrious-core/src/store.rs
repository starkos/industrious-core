// Copyright (c) 2025 Industrious One LLC
// SPDX-License-Identifier: MIT

use crate::Reducer;

pub struct Store<S, R> {
	_current_state: S,
	_root_reducer: R,
}

impl<S, R> Store<S, R>
where
	R: Reducer<S>,
{
	pub fn new(initial_state: S, root_reducer: R) -> Self {
		Self {
			_current_state: initial_state,
			_root_reducer: root_reducer,
		}
	}
}
