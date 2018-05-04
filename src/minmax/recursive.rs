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

	action.map.set_value((x as i128, y as i128), find_slot_player!(action.map.current_player));
	// action.map.number_captured((x as i32, y as i32), find_slots_players![action.map.current_player], true);
	action.map.change_player_turn();

	action
}

fn place_iterative(map: Map, x: usize, y: usize, alpha_beta: (i32, i32), depth: i32) -> Action
{
	let mut action = Action::new_iterative(map, (x, y), (alpha_beta.0, alpha_beta.1), depth);

	// let slot_player = &find_slot_player![action.map.current_player, Slot::PlayerOne, Slot::PlayerTwo];
	// let slot_enemy = &find_slot_enemy![action.map.current_player, Slot::PlayerOne, Slot::PlayerTwo];

	// action.map.is_winning_move(x, y);

	action.map.set_value((x as i128, y as i128), find_slot_player!(action.map.current_player));
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

fn select_best_aciton(action_1: &mut Action, action_2: Action, turn: &Turn)
{
	match *turn {
		Turn::MIN => {
			if action_2.value < action_1.beta
			{
				*action_1 = action_2;
				action_1.beta = action_1.value;
			}
		},
		Turn::MAX => {
			if action_2.value > action_1.alpha
			{
				*action_1 = action_2;
				action_1.alpha = action_1.value;
			}
		},
	}
}

#[allow(dead_code)]
fn solver_iterative(depth: i32, map: &mut Map, turn: Turn, alpha_beta: (i32, i32)) -> Option<Action>
{
	let mut go_stack: Vec<Action> = vec![];

	// let mut action_set = false;
	let mut current_elem: Action = Action::new_iterative(map.clone(), (0, 0), (alpha_beta.0, alpha_beta.1), depth);
	let mut new_action: Option<Action> = None;
	
	let mut dep;
	let mut new_trun: Turn = Turn::MAX;

	'start_of_loop: loop 
	{
		if current_elem.depth == 0
		{
									// last_action.value =	heuristic::map_value(map, (&Slot::PlayerTwo, &Slot::PlayerOne));
			current_elem.value = 0;
			current_elem.evaluate = true;
			match go_stack.pop()
			{
				Some(mut compare_action) => {
					if compare_action.evaluate == false
					{
						if current_elem.alpha < current_elem.beta
						{
							compare_action.action_done.push(current_elem);
							current_elem = compare_action; 
						}
					} 
					else if current_elem.depth != compare_action.depth
					{
						//check if current_elem.depth < compare_action.depth
						match new_trun {
							Turn::MIN => compare_action.beta = current_elem.value,
							Turn::MAX => compare_action.alpha = current_elem.value,
						};
						compare_action.value = current_elem.value;
						if compare_action.depth != depth
						{
							current_elem = compare_action;
						}
						new_trun = change_turn(&new_trun);
					}
					else
					{
						if current_elem.alpha < current_elem.beta
						{
							select_best_aciton(&mut current_elem, compare_action, &turn);
						}
					}
				} 
				_ => break 'start_of_loop,
			};
		}
		else if current_elem.evaluate == false
		{
			let area = map.area_of_interest();
			current_elem.evaluate = true;
			let new_map = current_elem.map.clone();
			let a = current_elem.alpha;
			let b = current_elem.beta;

			dep = current_elem.depth -1; // test if not move
			go_stack.push(current_elem);
			'root: for y_x in area.iter()
			{
				if map.is_available((y_x.1 , y_x.0)) == 0
				{
					match new_action {
						Some(action) => go_stack.push(action),
						_  => (),
					};
					new_action = Some(place_iterative(new_map.clone(), y_x.1 as usize , y_x.0 as usize, (a, b), dep));
					
				}
			}
			match new_action {
				Some(tmp_a)		=> { current_elem = tmp_a },
				_				=> current_elem = go_stack.pop().unwrap(), // need to replace for None
			};

			new_action = None;
			new_trun = change_turn(&new_trun);
		}
		else if current_elem.evaluate == true
		{
			if !current_elem.action_done.is_empty()
			{
				while let Some(tmp_action) = current_elem.action_done.pop()
				{
					if current_elem.alpha < current_elem.beta
					{
						select_best_aciton(&mut current_elem, tmp_action, &turn);
					}
				}
			}

			match go_stack.pop()
			{
				Some(mut compare_action) => {
					if compare_action.evaluate == false
					{
						if current_elem.alpha < current_elem.beta
						{
							compare_action.action_done.push(current_elem);
							current_elem = compare_action; 
						}
					} 
					else if current_elem.depth != compare_action.depth
					{
						//check if current_elem.depth < compare_action.depth
						match new_trun {
							Turn::MIN => compare_action.beta = current_elem.value,
							Turn::MAX => compare_action.alpha = current_elem.value,
						};
						compare_action.value = current_elem.value;
						if compare_action.depth != depth
						{
							current_elem = compare_action; 
						}
						new_trun = change_turn(&new_trun);
					}
					else
					{
						if current_elem.alpha < current_elem.beta
						{
							select_best_aciton(&mut current_elem, compare_action, &turn);
						}
					}
				} 
				_ => break 'start_of_loop,
			};
		}
	}

	return Some(current_elem);
}

#[allow(dead_code)]
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
	let depth: i32 = 3;

	// let action = solver(depth, &mut map.clone(), Turn::MAX, (MIN, MAX));
	let action = solver_iterative(depth, &mut map.clone(), Turn::MAX, (MIN, MAX));
	// let action = None;

	return action;
}
