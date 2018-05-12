
use std::string::String;
use piston_window::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
use goban::map::{Map};
use graphic::cursor::{ Cursor, Scene, Controls};
use goban::player::{ PlayerKind};
use graphic::draw::{ draw_text, Colors };

const BACKGROUND:[f32; 4] = [0.2, 0.2, 0.2, 1.0];

struct SettingsElem
{
	pub text: String,
	pub base: String,
	pub t: (f64, f64),
}

pub struct Settings
{
	gl: GlGraphics,
	index: usize,
	player_one: PlayerKind,
	player_two: PlayerKind,
	elems: Vec<SettingsElem>,
}

pub fn change_player_kind(player: &mut PlayerKind, level: i128)
{
	match player
	{
		PlayerKind::Human	  => *player = PlayerKind::AI(level),
		PlayerKind::AI { .. } => *player = PlayerKind::Human,
	}
}

pub fn reset_ai_level(player: &mut PlayerKind, level: i128)
{
	match player
	{
		PlayerKind::AI { .. }	=> *player = PlayerKind::AI(level),
		_						=> () 
	}
}

pub fn change_ai_level(ai_level: i128, cursor: &mut Cursor, player: &mut PlayerKind) -> i128
{
	let mut new_level: i128 = ai_level;

	if !cursor.prev
	{
		if ai_level == 10 {
			new_level = 1;
		} else {
			new_level = ai_level + 1;
		}
	}
	else if cursor.prev
	{
		if ai_level == 1 {
			new_level = 10;
		} else {
			new_level = ai_level - 1;
		}
		cursor.prev = false;
	}

	reset_ai_level(player, new_level);
	cursor.press = false;
	new_level
}

pub fn change_bool(elem_type: &mut bool)
{
	match elem_type
	{
		true	=> *elem_type = false,
		false	=> *elem_type = true,
	}
}

pub fn kind_to_str(player: &PlayerKind) -> String
{
	match player
	{
		PlayerKind::Human	  => "Human".to_owned(),
		PlayerKind::AI { .. } => "AI".to_owned(),
	}
}

impl Settings
{
	pub fn new(opengl: OpenGL, level2: i128) -> Self
	{
		let mut vect: Vec<SettingsElem> = vec![];
		vect.push( SettingsElem { text: "".to_owned(), base: "Hints : ".to_owned(), t: (300.0, 150.0)} );
		vect.push( SettingsElem { text: "".to_owned(), base: "Player One : ".to_owned(), t: (300.0, 200.0)} );
		vect.push( SettingsElem { text: "".to_owned(), base: "AI level player 1 : ".to_owned(), t: (300.0, 250.0)} );
		vect.push( SettingsElem { text: "".to_owned(), base: "Player Two : ".to_owned(), t: (300.0, 300.0)} );
		vect.push( SettingsElem { text: "".to_owned(), base: "AI level player 2 : ".to_owned(), t: (300.0, 350.0)} );
		vect.push( SettingsElem { text: "Start".to_owned(), base: "Start ".to_owned(), t: (375.0, 450.0)} );

		Settings {
			gl: GlGraphics::new(opengl),
			index: 5,
			player_one: PlayerKind::Human,
			player_two: PlayerKind::AI(level2),
			elems: vect,
		}
	}
	
	fn select_action(&mut self, cursor: &mut Cursor, map: &mut Map)
	{
		if cursor.press
		{
			match self
			{
				Settings {index, ..} if *index == 0 			=> change_bool(&mut cursor.hint),
				Settings {index, player_one, ..} if *index == 1 => change_player_kind(player_one, cursor.ai1_level),
				Settings {index, player_one, ..} if *index == 2	=> cursor.ai1_level = change_ai_level(cursor.ai1_level, cursor, player_one),
				Settings {index, player_two, ..} if *index == 3 => change_player_kind(player_two, cursor.ai2_level),
				Settings {index, player_two, ..} if *index == 4	=> cursor.ai2_level = change_ai_level(cursor.ai2_level, cursor, player_two),
				Settings {index, player_one, player_two, ..} if *index == 5	=> {
					map.reset();
					map.reset_players(player_one.clone(), player_two.clone());
					cursor.controller = Controls::GameControls;
					cursor.selected_scene = Scene::Game;
				},
				_  => (),
			};
			cursor.press = false;
		}
	}

	fn format_text(&mut self, cursor: &Cursor)
	{
		self.elems[0].text = format!("{}{}", self.elems[0].base, cursor.hint.to_owned());
		self.elems[1].text = format!("{}{}", self.elems[1].base, kind_to_str(&self.player_one));
		self.elems[2].text = format!("{}{}", self.elems[2].base, cursor.ai1_level.to_owned());
		self.elems[3].text = format!("{}{}", self.elems[3].base, kind_to_str(&self.player_two));
		self.elems[4].text = format!("{}{}", self.elems[4].base, cursor.ai2_level.to_owned());
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

	pub fn render(&mut self, args: &RenderArgs, mut glyph_cache: &mut GlyphCache, mut cursor: &mut Cursor, map: &mut Map) //RenderArgs
	{
		let index = self.select_index(&mut cursor);
		self.select_action(&mut cursor, map);
		self.format_text(&cursor);
		let vect = &self.elems;

		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);

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