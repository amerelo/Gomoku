use goban::map::{Map};

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
	pub fn new(map: Map, x_y: (usize, usize), alpha_beta: (i32, i32)) -> Self
	{
		Action {
			value: 0,
			alpha: alpha_beta.0,
			beta: alpha_beta.1,
			map: map,
			x_y: x_y,
			number_captured: 0,
		}
	}
}