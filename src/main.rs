
extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate fps_counter;
extern crate itertools;

#[macro_use]
mod goban;
mod minmax;
mod graphic;
mod heuristic;

use graphic::gomoku_graphic::start;

fn main()
{
	start();
}