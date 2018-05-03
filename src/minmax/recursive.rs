use std::i32::{MIN, MAX};

use minmax::action::{ Action };
use goban::map::{ Map };
use goban::player::{Player};
use heuristic;

#[derive(PartialEq)]
enum Turn
{
	MIN,
	MAX
}

fn change_turn(turn: &Turn) -> Turn
{
	match *turn {
		Turn::MIN => Turn::MAX,
		Turn::MAX => Turn::MIN,
	}
}

fn get_start_value(turn: &Turn) -> i32
{
	match *turn {
		Turn::MIN => MAX,
		Turn::MAX => MIN,
	}
}

fn try_place(map: Map, x: usize, y: usize) -> Action
{
	let mut action = Action::new(map, (x, y));

	// let slot_player = &find_slot_player![action.map.current_player, Slot::PlayerOne, Slot::PlayerTwo];
	// let slot_enemy = &find_slot_enemy![action.map.current_player, Slot::PlayerOne, Slot::PlayerTwo];

	// action.map.is_winning_move(x, y);

	action.map.set_value((x as i64, y as i64), find_slot_player!(action.map.current_player));
	// action.map.number_captured((x as i32, y as i32), find_slots_players![action.map.current_player], true);
	action.map.change_player_turn();

	action
}

fn best_action(turn: &Turn, new_action: Action, best_action: &mut Action) // -> Option<Action>
{
	match *turn {
		Turn::MIN => {
			if  new_action.value < best_action.value
			{
				*best_action =  new_action;
			}
		},
		Turn::MAX => {
			if  new_action.value > best_action.value
			{
				*best_action = new_action;
			}
		},
	}
}

fn solver(depth: i32, map: &mut Map, turn: Turn) -> Action //Option<Action>
{
	if depth == 0
	{
		let mut last_action: Action = Action::new(map.clone(), (0, 0));
													  // first slot is for the player we want the score
		// last_action.value =	heuristic::map_value(map, (&Slot::PlayerTwo, &Slot::PlayerOne));
		last_action.value =	0;

		return last_action;
	}
	let mut best: Action = Action::new(map.clone(), (0, 0));
	best.value = get_start_value(&turn);

	for (y, _elem_y) in map.value.iter().enumerate()
	{
		for x in 0..19
		{
			if map.is_available((x as i64, y as i64)) == 0
			{
				let new_trun: Turn = change_turn(&turn);
				let mut new_map = map.clone();
				let mut new_action = try_place(new_map, x, y);

				new_action.value = solver(depth - 1, &mut new_action.map, new_trun).value;
				best_action(&turn, new_action, &mut best);
			}
		}
	}
	return best;
}

pub fn start_min_max(map: &Map) -> Action
{
	let depth: i32 = 3;

	let action = solver(depth, &mut map.clone(), Turn::MAX);
	println!("x = {}, y = {}", action.x_y.0, action.x_y.1);
	return action;
}