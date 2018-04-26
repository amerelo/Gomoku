extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate fps_counter;

mod graphic;
use graphic::gomoku_graphic::start;

// mod gomoku;
// use gomoku::map::{Map, Slot};
// use gomoku::player::{Player, PlayerKind};

fn main()
{
	start();

	// let mut test = Map {..Default::default() };
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
