use std::time::{Instant};
use fps_counter::FPSCounter;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
use piston_window::*;

use goban::player::{Player, PlayerKind};
use goban::map::{ Map };
use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor };
use graphic::draw::{ draw_goban, draw_player, draw_text , Colors};
use minmax::recursive::{ start_min_max };

const BACKGROUND:[f32; 4] = [0.65, 0.55, 0.45, 1.0];
// 0.95, 0.69, 0.50

pub struct Game {
	pub fps: FPSCounter,
	pub gl: GlGraphics, // OpenGL drawing backend.
	pub goban: GoElem,
	pub go_w: GoElem,
	pub go_b: GoElem,
	pub map: Map,
	pub my_time: f64,
}

impl Game
{
	pub fn new(opengl: OpenGL) -> Self
	{
		Game {
			fps: FPSCounter::new(),
			gl: GlGraphics::new(opengl),
			map: Map {..Default::default() },
			goban: GoElem::new("resources/goban.png", 1.5),
			go_b: GoElem::new("resources/w_1.png", 0.09),
			go_w: GoElem::new("resources/black.png", 0.10),
			my_time: 0.0,
		}
	}

	pub fn render(&mut self, args: &RenderArgs, mut glyph_cache: &mut GlyphCache, mut cursor: &mut Cursor) //RenderArgs
	{
		let goban = &self.goban;
		let map = &mut self.map;
		let players = (&self.go_w, &self.go_b);

		let fps_t = &format!("fps: {}            time of last AI move: {:.5} ms", self.fps.tick(), self.my_time);
		// let turn = ;

		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);

			draw_text(gl, &mut glyph_cache, fps_t, c.transform.trans(5.0, 20.0), Colors::NORMAL);
			draw_text(gl, &mut glyph_cache, &format!("Turn: {}", map.turn), c.transform.trans(5.0, 40.0), Colors::NORMAL);
			draw_goban(c, gl, goban);
			draw_player(c, gl, map, &mut cursor, players);
		});

		// let player_turn = find_slot_player!(map.current_player, Slot::PlayerOne, Slot::PlayerTwo);
		if find_kind_player![map.current_player, map.players_kind] == &PlayerKind::AI
		{
			let now = Instant::now();
			match start_min_max(&map)
			{
				Some(action) => {
					map.number_captured((action.x_y.0 as i128, action.x_y.1 as i128), find_slot_player![map.current_player], true);
					map.set_value((action.x_y.0 as i128, action.x_y.1 as i128), find_slot_player!(map.current_player));
					// map.change_player_turn();
				},
				None => (),
			}
			map.change_player_turn();
			let elapsed = now.elapsed();
			let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
			self.my_time = sec;
		} 
		else if !cursor.press && cursor.place_piece &&
			map.is_available((cursor.cursor_in_board[0] as i128, cursor.cursor_in_board[1] as i128)) == 0
		{

			// map.is_winning_move((cursor.cursor_in_board[0] as i32, cursor.cursor_in_board[1] as i32));
			
			// println!("value {}\n", heuristic::value_slot(map, (cursor.cursor_in_board[1] as i128, cursor.cursor_in_board[0] as i128, 2)));
			map.number_captured((cursor.cursor_in_board[0] as i128, cursor.cursor_in_board[1] as i128), find_slot_player![map.current_player], true);
			
			map.set_value((cursor.cursor_in_board[0] as i128, cursor.cursor_in_board[1] as i128), find_slot_player!(map.current_player));
			map.change_player_turn();

			cursor.place_piece = false;
		}
	}
}
