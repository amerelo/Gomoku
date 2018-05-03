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


fn best_action(turn: Turn, tmp_vec: Vec<Action>) -> Action
{
	if tmp_vec.is_empty()
	{
		println!("empty vec ----------------------------- :(");
	}

	let mut action = Action::new(tmp_vec[0].map.clone() , tmp_vec[0].x_y);

	match turn 
	{
		Turn::MAX  => {
			for elem in tmp_vec.iter()
			{
				if elem.value > action.value
				{
					action = elem.clone();
				}
			}
		}
		_		=> {
			for elem in tmp_vec.iter()
			{
				if elem.value < action.value
				{
					action = elem.clone();
				}
			}
		}
	}
	action
}

fn solver(depth: i32, map: &mut Map, turn: Turn) -> Action
{
	if depth == 0
	{
		let mut last_action: Action = Action::new(map.clone(), (0, 0));
													  // first slot is for the player we want the score
		// last_action.value =	heuristic::map_value(map, (&Slot::PlayerTwo, &Slot::PlayerOne));
		last_action.value =	0;

		return last_action;
	}
	let mut tmp_vec: Vec<Action> = vec![];

	for (y, _elem_y) in map.value.iter().enumerate()
	{
		for x in 0..19
		{
			if map.is_available((x as i64, y as i64)) == 0
			{
				let mut new_map = map.clone();
				let mut new_trun: Turn;
				match turn {
					Turn::MIN => new_trun = Turn::MAX,
					_ 		  => new_trun = Turn::MIN
				}
				let mut new_action = try_place(new_map, x, y);
				new_action.value = solver(depth - 1, &mut new_action.map, new_trun).value;
				// println!("x_y {:?}", new_action.x_y);
				tmp_vec.push(new_action);
			}
		}
	}

	// println!(">>>>>>>>>>>>>>.. ===== {:?}", tmp_vec);

	return best_action(turn, tmp_vec);
}

pub fn start_min_max(map: &Map) -> Action
{
	let depth: i32 = 3;

	let action = solver(depth, &mut map.clone(), Turn::MAX);
	println!("x = {}, y = {}", action.x_y.0, action.x_y.1);
	return action;
}