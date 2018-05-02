use goban::map::{Map};

pub struct Node
{
	value: i32,
	map: Map,
	alpha: i32,
	beta: i32,
}