
extern crate piston;
extern crate sdl2_window;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate fps_counter;

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