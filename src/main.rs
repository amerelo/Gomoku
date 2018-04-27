extern crate piston;
extern crate graphics;
// extern crate glutin_window;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate fps_counter;
// extern crate piston_window;

mod graphic;
mod goban;

use graphic::gomoku_graphic::start;

use goban::map::{Map, Slot};
use goban::player::{Player, PlayerKind};

fn main()
{
	start();
	// let mut map = Map {..Default::default() };

	// map.value[3][1] = Slot::PlayerOne;
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
