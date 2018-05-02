use goban::map::{Map};
use std::i32::{MIN, MAX};

#[derive(Debug, Clone)]
pub struct Action
{
	pub value: i32,
	pub alpha: i32,
	pub beta: i32,
	pub map: Map,
	pub x_y: (usize, usize),
	pub number_captured: usize,
}

impl Action
{
	pub fn new(map: Map, x_y: (usize, usize)) -> Self
	{
		Action {
			value: 0,
			alpha: MIN,
			beta: MAX,
			map: map,
			x_y: x_y,
			number_captured: 0,
		}
	}
}