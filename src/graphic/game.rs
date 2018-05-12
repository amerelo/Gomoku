use std::time::{Instant};
use fps_counter::FPSCounter;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
use piston_window::*;

use goban::player::{Player, PlayerKind};
use goban::finish::{ Finish };
use goban::map::{ Map };
use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor , Scene, Controls};
use graphic::draw::{ draw_goban, draw_player, draw_text, draw_hint , Colors};
use minmax::recursive::{ start_min_max };

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::prelude::*;

const BACKGROUND:[f32; 4] = [0.65, 0.55, 0.45, 1.0];
const BLACK:[f32; 4] = [0.1, 0.1, 0.1, 0.9];

// 0.95, 0.69, 0.50

struct SettingsElem
{
	pub text: String,
	pub base: String,
	pub t: (f64, f64),
}

pub struct Game {
	pub fps: FPSCounter,
	pub gl: GlGraphics, // OpenGL drawing backend.
	pub goban: GoElem,
	pub go_w: GoElem,
	pub go_b: GoElem,
	pub map: Map,
	pub file: File,
	pub my_time: f64,
	index: usize,
	elems: Vec<SettingsElem>,
}

impl Game
{
	pub fn new(opengl: OpenGL) -> Self
	{
		let mut vect: Vec<SettingsElem> = vec![];
		vect.push( SettingsElem { text: "Main Menu".to_owned(), base: "Main Menu".to_owned(), t: (300.0, 300.0)} );
		vect.push( SettingsElem { text: "Retry ".to_owned(), base: "Retry ".to_owned(), t: (300.0, 400.0)} );
		
		Game {
			fps: FPSCounter::new(),
			gl: GlGraphics::new(opengl),
			map: Map {..Default::default() },
			goban: GoElem::new("resources/goban.png", 1.5),
			go_b: GoElem::new("resources/w_1.png", 0.09),
			go_w: GoElem::new("resources/black.png", 0.10),
			file: File::create(format!["./save/{:?}.save", SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()]).expect("Unable to create file"),
			my_time: 0.0,
			index: 1,
			elems: vect,
		}
	}

	fn select_index(&mut self, cursor: &mut Cursor) -> usize
	{
		let i = self.index;
		let len = self.elems.len();

		if cursor.up
		{
			if i == 0 {
				self.index = len - 1;
			} else {
				self.index = i - 1;
			}
			cursor.up = false;
		}
		else if cursor.down
		{
			self.index = (i + 1) % len;
			cursor.down = false;
		}
		self.index
	}

	pub fn render(&mut self, args: &RenderArgs, mut glyph_cache: &mut GlyphCache, cursor: &mut Cursor, list_of_maps: &mut Vec<Map>) //RenderArgs
	{
		let index = self.select_index(cursor);

		
		if self.map.is_finish != Finish::None
		{
			self.select_action(&index, cursor);
		}

		let mut map = &mut self.map;
		let goban = &self.goban;
		let players = (&self.go_w, &self.go_b);

		let fps_t = &format!("fps: {}            Time of last AI move: {:.5} s", self.fps.tick(), self.my_time);
		let vect = &self.elems;
		let pc_1 = &format!("P1 {}", map.players_score.0);
		let pc_2 = &format!("P2 {}", map.players_score.1);

		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);

			draw_text(gl, &mut glyph_cache, fps_t, c.transform.trans(5.0, 20.0), Colors::BLACK);
			draw_text(gl, &mut glyph_cache, &format!("Turn: {}", map.turn), c.transform.trans(5.0, 38.0), Colors::BLACK );
			draw_text(gl, &mut glyph_cache, pc_1, c.transform.trans(5.0, 60.0), Colors::BLACK);
			draw_text(gl, &mut glyph_cache, pc_2, c.transform.trans(5.0, 80.0), Colors::BLACK);
			draw_goban(c, gl, goban);
			draw_player(c, gl, &mut map, cursor, players);
			
			draw_hint(c, gl, &mut map, &mut glyph_cache);

			if map.is_finish != Finish::None
			{
				end_menu(c, gl, &mut glyph_cache, &map, &vect, index)
			}
		});
		game_action(&mut map, cursor, list_of_maps, &mut self.my_time, &mut self.file);
	}

	pub fn new_file(&mut self) -> ()
	{
		self.file = File::create(format!["./save/{:?}.save", SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()]).expect("Unable to create file");
	}

	fn select_action(&mut self, index: &usize, cursor: &mut Cursor)
	{
		if cursor.press && *index == 0 {
			self.map.reset();
			self.new_file();
			cursor.selected_scene = Scene::Settings;
			cursor.controller = Controls::KeyBoard;
			cursor.press = false;
		}
		else if cursor.press && *index == 1 {
			self.map.reset();
			self.new_file();
			cursor.selected_scene = Scene::Game;
			cursor.controller = Controls::GameControls;
			cursor.press = false;
		}
	}

}

fn ai_move(map: &mut Map, my_time: &mut f64, file: &mut File)
{
	let now = Instant::now();
	match start_min_max(&map)
	{
		Some(action) => {
			map.number_captured((action.x_y.0 as i128, action.x_y.1 as i128), find_slot_player![map.current_player], true);
			map.set_value((action.x_y.0 as i128, action.x_y.1 as i128), find_slot_player!(map.current_player));
			map.five_align();
		},
		None => (),
	}
	backtrace(file, map);
	map.change_player_turn();
	let elapsed = now.elapsed();
	let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
	*my_time = sec;
}

fn human_move(map: &mut Map, cursor: &mut Cursor, file: &mut File)
{
	map.number_captured((cursor.cursor_in_board[0] as i128, cursor.cursor_in_board[1] as i128), find_slot_player![map.current_player], true);
	map.set_value((cursor.cursor_in_board[0] as i128, cursor.cursor_in_board[1] as i128), find_slot_player!(map.current_player));
	map.five_align();
	backtrace(file, map);
	map.change_player_turn();

	cursor.place_piece = false;
}

fn game_action(map: &mut Map, cursor: &mut Cursor, list_of_maps: &mut Vec<Map>, my_time: &mut f64, file: &mut File)
{
	if cursor.undo
	{
		let mut ai_count = 0;
		if find_kind_player![map.current_player, map.players_kind] == &PlayerKind::AI
		{
			ai_count = ai_count + 1;
		}
		if find_kind_player![find_kind_enemy!(map.current_player) , map.players_kind] == &PlayerKind::AI
		{
			ai_count = ai_count + 1;
		}

		if ai_count == 1
		{
			let _ = list_of_maps.pop();
		}
		if ai_count <= 1
		{
			if let Some(lastmove) = list_of_maps.pop()
			{
				*map = lastmove;
			}
		}
		cursor.undo = false;
	}

	if map.is_finish != Finish::None
	{
		*my_time = 0.0;
		cursor.controller = Controls::KeyBoard;
	}
	else if find_kind_player![map.current_player, map.players_kind] == &PlayerKind::AI
	{
		list_of_maps.push(map.clone());
		ai_move(map, my_time, file);
	} 
	else if !cursor.press && cursor.place_piece &&
		map.is_available((cursor.cursor_in_board[0] as i128, cursor.cursor_in_board[1] as i128), &map.current_player) == 0
	{
		list_of_maps.push(map.clone());
		human_move(map, cursor, file);
	}
}

pub fn player_win(map: &Map) -> String
{
	match map.is_finish
	{
		Finish::CapturePlayerOne	=> "Player 1 Win by Capture".to_owned(),
		Finish::CapturePlayerTwo	=> "Player 2 Win by Capture".to_owned(),
		Finish::AlignPlayerOne		=> "Player 1 Win by Align".to_owned(),
		Finish::AlignPlayerTwo		=> "Player 2 Win by Align".to_owned(),
		_							=> "None".to_owned(),
	}
}

fn end_menu(c: Context, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache, map: &Map, vect: &Vec<SettingsElem>, index: usize)
{
	let square = rectangle::square(0.0, 0.0, 400.0);
	
	let winner = player_win(&map);

	rectangle(BLACK, square, c.transform.trans(200.0, 130.0), gl);
	draw_text(gl, glyph_cache, &winner ,c.transform.trans(270.0, 200.0), Colors::RED);
	for (i, elem) in vect.iter().enumerate()
	{
		if index == i {
			draw_text(gl, glyph_cache, &elem.text, c.transform.trans(elem.t.0, elem.t.1), Colors::Yellow);
		} else {
			draw_text(gl, glyph_cache, &elem.text, c.transform.trans(elem.t.0, elem.t.1), Colors::NORMAL);
		}
	}
}

fn backtrace(file: &mut File, map: &mut Map) -> ()
{
	file.write_all(map.to_string().as_bytes()).expect("not working");
}
