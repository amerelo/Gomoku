
extern crate piston;
// extern crate glutin_window;
extern crate sdl2_window;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;

extern crate fps_counter;

#[macro_use]
mod goban;
mod graphic;
use graphic::gomoku_graphic::start;

mod heuristic;
use goban::map::{Map, slot::Slot};
// use goban::player::{Player, PlayerKind};

fn main()
{
	// start();
	let mut map = Map {..Default::default() };
	println!("{}", heuristic::current_value(&mut map, (3, 2), (&Slot::PlayerOne, &Slot::PlayerTwo)));
	map.value[1][3] = Slot::PlayerOne;
	map.value[3][3] = Slot::PlayerOne;
	println!("{}", heuristic::current_value(&mut map, (3, 2), (&Slot::PlayerOne, &Slot::PlayerTwo)));
	// map.value[5][1] = Slot::PlayerOne;
	// map.value[4][3] = Slot::PlayerOne;
	// map.value[4][2] = Slot::PlayerOne;

	// test.value[0][3] = Slot::Used(Player::One(PlayerKind::AI));
	// test.value[3][0] = Slot::Used(Player::One(PlayerKind::AI));
	// test.value[2][3] = Slot::Used(Player::One(PlayerKind::AI));
	// test.value[12][3] = Slot::Used(Player::Two(PlayerKind::Human));
	// test.value[13][4] = Slot::Used(Player::One(PlayerKind::AI));
	// test.value[2][5] = Slot::Used(Player::One(PlayerKind::AI));
	// test.value[11][3] = Slot::Used(Player::One(PlayerKind::AI));
	// test.value[18][3] = Slot::Used(Player::One(PlayerKind::AI));
	// for t in &test.value
	// {
	// 	println!("{} {:?}", t.len(), t);
	// }
}


// [dependencies.pistoncore-sdl2_window]
// git = "https://github.com/PistonDevelopers/sdl2_window"