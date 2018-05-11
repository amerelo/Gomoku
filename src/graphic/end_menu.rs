
use std::string::String;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
use goban::map::{Map};
use graphic::cursor::{ Cursor, Scene};
use graphic::draw::{ draw_text, Colors };
use goban::finish::{ Finish };

const BACKGROUND:[f32; 4] = [0.2, 0.2, 0.2, 1.0];

struct SettingsElem
{
	pub text: String,
	pub base: String,
	pub t: (f64, f64),
}

pub struct EndMenu
{
	gl: GlGraphics,
	index: usize,
	elems: Vec<SettingsElem>,
}

impl EndMenu
{
	pub fn new(opengl: OpenGL) -> Self
	{
		let mut vect: Vec<SettingsElem> = vec![];
		vect.push( SettingsElem { text: "Main Menu".to_owned(), base: "Main Menu".to_owned(), t: (350.0, 300.0)} );
		vect.push( SettingsElem { text: "Retry ".to_owned(), base: "Retry ".to_owned(), t: (350.0, 400.0)} );

		EndMenu {
			gl: GlGraphics::new(opengl),
			index: 1,
			elems: vect,
		}
	}
	
	fn select_action(&mut self, cursor: &mut Cursor, map: &mut Map)
	{
		if cursor.press
		{
			match self
			{
				EndMenu {index, ..}  if *index == 0 => {
					map.reset();
					cursor.selected_scene = Scene::Game;
				},
				EndMenu {index, ..} if *index == 1  => {
					map.reset();
					cursor.selected_scene = Scene::Settings;
				},
				_  => (),
			};
			cursor.press = false;
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

	pub fn render(&mut self, args: &RenderArgs, mut glyph_cache: &mut GlyphCache, mut cursor: &mut Cursor, map: &mut Map, list_of_maps: &mut Vec<Map>) //RenderArgs
	{
		let index = self.select_index(&mut cursor);
		self.select_action(&mut cursor, map);
		
		let winner = player_win(&map);
		let vect = &self.elems;

		for elem in list_of_maps.iter()
		{
			elem.print_map();
			println!("------------------------------------------------------");
		}
		list_of_maps.clear();

		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);

			draw_text(gl, &mut glyph_cache, &winner ,c.transform.trans(270.0, 200.0), Colors::RED);
			for (i, elem) in vect.iter().enumerate()
			{
				if index == i {
					draw_text(gl, &mut glyph_cache, &elem.text, c.transform.trans(elem.t.0, elem.t.1), Colors::Yellow);
				} else {
					draw_text(gl, &mut glyph_cache, &elem.text, c.transform.trans(elem.t.0, elem.t.1), Colors::NORMAL);
				}
			}
		});
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

