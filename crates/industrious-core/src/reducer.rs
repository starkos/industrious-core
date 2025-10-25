// Copyright (c) 2025 Industrious One LLC
// SPDX-License-Identifier: MIT

use std::any::Any;

pub trait Reducer<S> {
	fn reduce(&self, state: &S, action: &dyn Any) -> S;
}

impl<F, S> Reducer<S> for F
where
	F: Fn(&S, &dyn Any) -> S,
{
	fn reduce(&self, state: &S, action: &dyn Any) -> S {
		self(state, action)
	}
}
