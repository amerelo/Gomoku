use std::i128::{MIN, MAX};

use minmax::action::{ Action };
use goban::map::{ Map };
use goban::player::{Player};
use heuristic;

const MAX_VEC_AREA: usize = 30;
const DEAPH: usize = 2;

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

fn place(map: Map, x: usize, y: usize, alpha_beta: (i128, i128)) -> Action
{
	let mut action = Action::new(map, (x, y), (alpha_beta.0, alpha_beta.1));
	let slot_player = find_slot_player![action.map.current_player];

	// action.map.is_winning_move(x, y);
			
	action.map.number_captured((x as i128, y as i128), slot_player, true);
	action.map.set_value((x as i128, y as i128), slot_player);
	// action.map.number_captured((x as i128, y as i128), find_slots_players![action.map.current_player], true);
	action.map.change_player_turn();

	action
}

fn place_iterative(map: Map, x: usize, y: usize, alpha_beta: (i128, i128), depth: i128) -> Action
{
	let mut action = Action::new_iterative(map, (x, y), (alpha_beta.0, alpha_beta.1), depth);
	let slot_player = find_slot_player![action.map.current_player];

	// action.map.is_winning_move(x, y);
	action.map.number_captured((x as i128, y as i128), slot_player, true);
	action.map.set_value((x as i128, y as i128), slot_player);
	// action.map.number_captured((x as i128, y as i128), find_slots_players![action.map.current_player], true);
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

fn select_best_action(action_1: &mut Action, action_2: Action, turn: &Turn)
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
fn solver_iterative(depth: i128, map: &mut Map, turn: Turn, alpha_beta: (i128, i128)) -> Option<Action>
{
	let mut go_stack: Vec<Action> = vec![];

	// let mut action_set = false;
	let mut current_elem: Action = Action::new_iterative(map.clone(), (0, 0), (alpha_beta.0, alpha_beta.1), depth);
	let mut new_action: Option<Action> = None;
	
	let mut dep;
	let mut current_trun: Turn = turn;

	'start_of_loop: loop 
	{
		if current_elem.depth == 0
		{
			current_elem.value = heuristic::value_map(&current_elem.map, &Player::Two);
			current_elem.evaluate = true;
			match go_stack.pop()
			{
				Some(mut compare_action) => {
					if compare_action.evaluate == false
					{
						if current_elem.depth != compare_action.depth
						{
							println!("current elem {} -- compare elem {}",  current_elem.depth, compare_action.depth);
						} // need this to see if some bug appear

						compare_action.value = heuristic::value_map(&current_elem.map, &Player::Two);
						compare_action.evaluate = true;

						select_best_action(&mut current_elem, compare_action, &current_trun);
					} 
					else if current_elem.depth != compare_action.depth
					{
						//check if current_elem.depth < compare_action.depth
						match current_trun {
							Turn::MIN => compare_action.beta = current_elem.value,
							Turn::MAX => compare_action.alpha = current_elem.value,
						};
						compare_action.value = current_elem.value;
						if compare_action.depth != depth
						{
							current_elem = compare_action;
						}
						else {
							new_action = Some(current_elem);
							break 'start_of_loop;
						}
						current_trun = change_turn(&current_trun);
					}
					else if current_elem.alpha < current_elem.beta
					{
						select_best_action(&mut current_elem, compare_action, &current_trun);
					}
				} 
				_ => {
					new_action = None;
					break 'start_of_loop
				},
			};
		}
		else if current_elem.evaluate == false
		{
			let area = current_elem.map.area_of_interest(MAX_VEC_AREA - DEAPH, &map.current_player);
			// println!("new evale depth {} --- area size {}", current_elem.depth, area.len());
			
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
					// println!("new action evaluate ? {:?}", new_action);
				}
				// println!("_________________________________________________________________________________-");
			}
			// println!("------------------------------------------------------------------------");
			match new_action {
				Some(tmp_a)		=> { current_elem = tmp_a },
				_				=> current_elem = go_stack.pop().unwrap(), // need to replace for None
			};

			new_action = None;
			current_trun = change_turn(&current_trun);
		}
		else if current_elem.evaluate == true
		{
			if !current_elem.action_done.is_empty()
			{
				while let Some(tmp_action) = current_elem.action_done.pop()
				{
					if current_elem.alpha < current_elem.beta
					{
						select_best_action(&mut current_elem, tmp_action, &current_trun);
					}
				}
			}

			match go_stack.pop()
			{
				Some(mut compare_action) => {
					if compare_action.evaluate == false
					{
						if current_elem.depth != compare_action.depth
						{
							println!("{}", "error in logique");
						}

						if current_elem.alpha < current_elem.beta
						{
							compare_action.action_done.push(current_elem);
							current_elem = compare_action; 
						}
					} 
					else if current_elem.depth != compare_action.depth
					{
						//check if current_elem.depth < compare_action.depth
						match current_trun {
							Turn::MIN => compare_action.beta = current_elem.value,
							Turn::MAX => compare_action.alpha = current_elem.value,
						};
						compare_action.value = current_elem.value;
						if compare_action.depth != depth
						{
							current_elem = compare_action; 
						}
						else {
							new_action = Some(current_elem);
							break 'start_of_loop;
						}
						current_trun = change_turn(&current_trun);
					}
					else if current_elem.alpha < current_elem.beta
					{
						select_best_action(&mut current_elem, compare_action, &current_trun);
					}
				} 
				_ => {
					new_action = None;
					break 'start_of_loop;
				},
			};
		}
	}

	return new_action;
}

#[allow(dead_code)]
fn solver(depth: i128, map: &mut Map, turn: Turn, alpha_beta: (i128, i128)) -> Option<Action>
{
	if depth == 0
	{
		let mut last_action: Action = Action::new(map.clone(), (0, 0), (alpha_beta.0, alpha_beta.1));
													  // first slot is for the player we want the score
		last_action.value =	0;

		return Some(last_action);
	}

	let mut action_set = false;
	let mut tmp: Action = Action::new(map.clone(), (0, 0), (alpha_beta.0, alpha_beta.1));
	let current_trun: Turn = change_turn(&turn);
	
	let area = map.area_of_interest(MAX_VEC_AREA - DEAPH, &map.current_player);

	'root: for y_x in area.iter()
	{
		if map.is_available((y_x.1 , y_x.0)) == 0
		{
			let mut new_map = map.clone();
			let mut new_action = place(new_map, y_x.1 as usize , y_x.0 as usize, (tmp.alpha, tmp.beta));

			match solver(depth - 1, &mut new_action.map, current_trun.clone(), (new_action.alpha, new_action.beta))
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
	let depth: i128 = DEAPH as i128;

	// let action = solver(depth, &mut map.clone(), Turn::MAX, (MIN, MAX));
	let action = solver_iterative(depth, &mut map.clone(), Turn::MIN, (MIN, MAX));
	// let action = None;

	match &action
	{
		Some(_t) => (),
		_ => println!("ERROR no NULL return" ), 
	}


	return action;
}
