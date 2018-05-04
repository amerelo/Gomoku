use std::i32::{MIN, MAX};

use minmax::action::{ Action };
use goban::map::{ Map };
use goban::player::{Player};
use heuristic;

#[derive(PartialEq, Clone)]
pub enum Turn
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

fn place(map: Map, x: usize, y: usize, alpha_beta: (i32, i32)) -> Action
{
	let mut action = Action::new(map, (x, y), (alpha_beta.0, alpha_beta.1));

	// let slot_player = &find_slot_player![action.map.current_player, Slot::PlayerOne, Slot::PlayerTwo];
	// let slot_enemy = &find_slot_enemy![action.map.current_player, Slot::PlayerOne, Slot::PlayerTwo];

	// action.map.is_winning_move(x, y);

	action.map.set_value((x as i64, y as i64), find_slot_player!(action.map.current_player));
	// action.map.number_captured((x as i32, y as i32), find_slots_players![action.map.current_player], true);
	action.map.change_player_turn();

	action
}

fn best_action(turn: &Turn, new_action: Action, tmp: &mut Action, action_set: &mut bool)
{
	match *turn {
		Turn::MIN => {
			if  new_action.value < tmp.beta
			{
				*tmp = new_action;
				tmp.beta = tmp.value;
				*action_set = true;
			}
		},
		Turn::MAX => {
			if  new_action.value > tmp.alpha
			{
				*tmp = new_action;
				tmp.alpha = tmp.value;
				*action_set = true;
			}
		},
	}
}

fn solver(depth: i32, map: &mut Map, turn: Turn, alpha_beta: (i32, i32)) -> Option<Action>
{
	if depth == 0
	{
		let mut last_action: Action = Action::new(map.clone(), (0, 0), (alpha_beta.0, alpha_beta.1));
													  // first slot is for the player we want the score
		// last_action.value =	heuristic::map_value(map, (&Slot::PlayerTwo, &Slot::PlayerOne));
		last_action.value =	0;

		return Some(last_action);
	}

	let mut action_set = false;
	let mut tmp: Action = Action::new(map.clone(), (0, 0), (alpha_beta.0, alpha_beta.1));
	let new_trun: Turn = change_turn(&turn);
	
	let area = map.area_of_interest();

	'root: for y_x in area.iter()
	{
		if map.is_available((y_x.1 , y_x.0)) == 0
		{
			let mut new_map = map.clone();
			let mut new_action = place(new_map, y_x.1 as usize , y_x.0 as usize, (tmp.alpha, tmp.beta));

			match solver(depth - 1, &mut new_action.map, new_trun.clone(), (new_action.alpha, new_action.beta))
			{
				Some(action) => {
					new_action.value = action.value;
					best_action(&turn, new_action, &mut tmp, &mut action_set)
				},
				None => (),
			}

			if tmp.alpha >= tmp.beta
			{
				break 'root;
			}
		}
	}

	if action_set == true
	{
		return Some(tmp);
	}
	None
}

pub fn start_min_max(map: &Map) -> Option<Action>
{
	let depth: i32 = 1;

	let action = solver(depth, &mut map.clone(), Turn::MAX, (MIN, MAX));
	// let action = None;

	return action;
}
