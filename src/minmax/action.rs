use goban::map::{Map};

#[derive(Debug, Clone)]
pub struct Action
{
	pub value: i128,
	pub alpha: i128,
	pub beta: i128,
	pub depth: i128,
	pub number_captured: usize,
	pub x_y: (usize, usize),
	pub evaluate: bool,
	pub map: Map,
	pub action_done: Vec<Action>,
}

impl Action
{
	pub fn new(map: Map, x_y: (usize, usize), alpha_beta: (i128, i128)) -> Self
	{
		Action {
			value: 0,
			alpha: alpha_beta.0,
			beta: alpha_beta.1,
			map: map,
			x_y: x_y,
			number_captured: 0,
			evaluate: false,
			depth: 0,
			action_done: vec![],
		}
	}

	pub fn new_iterative(map: Map, x_y: (usize, usize), alpha_beta: (i128, i128), depth: i128) -> Self
	{
		Action {
			value: 0,
			alpha: alpha_beta.0,
			beta: alpha_beta.1,
			map: map,
			x_y: x_y,
			number_captured: 0,
			evaluate: false,
			depth: depth,
			action_done: vec![],
		}
	}
}