use graphics::*;

use opengl_graphics::{ GlGraphics, GlyphCache };
use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor };
use goban::map::{Map};
use goban::player::{Player};

use heuristic;

const SIZE_TEXT: u32 = 19;

const GOBANPOS: (f64, f64) = (70.0, 40.0);
const GOBAN_BOARD_X: f64 = 8.0;
const GOBAN_BOARD_Y: f64 = 10.0;
const GOBAN_SPACE: f64 = 34.5;

const COLOR_WS: [f32; 4] = [1.0, 1.0, 1.0, 0.6];
const COLOR_W: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const COLOR_R: [f32; 4] = [1.0, 0.0, 0.0, 0.6];
const COLOR_Y: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const COLOR_B: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub enum Colors
{
	SHADOW,
	NORMAL,
	BLACK,
	RED,
	Yellow,
}

pub fn draw_text(gl: &mut GlGraphics, glyph_cache: &mut GlyphCache, my_text: &str, transform: [[f64; 3]; 2], color: Colors)
{
	let _success;
	
	match color
	{
		Colors::NORMAL	=> _success = text(COLOR_W, SIZE_TEXT, my_text, glyph_cache, transform, gl),
		Colors::RED		=> _success = text(COLOR_R, SIZE_TEXT, my_text, glyph_cache, transform, gl),
		Colors::BLACK	=> _success = text(COLOR_B, SIZE_TEXT, my_text, glyph_cache, transform, gl),
		Colors::Yellow	=> _success = text(COLOR_Y, SIZE_TEXT, my_text, glyph_cache, transform, gl),
		_				=> (), 
	};
}

pub fn draw_goban(c: Context, gl: &mut GlGraphics, goban: &GoElem)
{
	let (newx, newy) = (GOBANPOS.0, GOBANPOS.1);

	let transform2 = c.transform.trans(newx, newy).scale(goban.scale, goban.scale);
	image(&goban.elem, transform2, gl);
}

fn draw_img(gl: &mut GlGraphics, player: &GoElem, transform: [[f64; 3]; 2], cl: Colors)
{
	match cl 
	{
		Colors::SHADOW => Image::new_color(COLOR_WS).draw(&player.elem, &DrawState::new_alpha(), transform, gl),
		Colors::NORMAL => Image::new_color(COLOR_W).draw(&player.elem, &DrawState::new_alpha(), transform, gl),
		Colors::RED    => Image::new_color(COLOR_R).draw(&player.elem, &DrawState::new_alpha(), transform, gl),
		Colors::Yellow => Image::new_color(COLOR_Y).draw(&player.elem, &DrawState::new_alpha(), transform, gl),
		Colors::BLACK  => Image::new_color(COLOR_B).draw(&player.elem, &DrawState::new_alpha(), transform, gl),
	}
}

fn draw_shadow(c: Context, gl: &mut GlGraphics, players: (&GoElem, &GoElem), near_pos: [f64; 2], slot: i64)
{
	match slot
	{
		1 => {
			let transform = c.transform.trans(near_pos[0], near_pos[1]).scale(players.0.scale, players.0.scale);
			draw_img(gl, players.0, transform, Colors::SHADOW);
		},
		_ => {
			let transform = c.transform.trans(near_pos[0], near_pos[1]).scale(players.1.scale, players.1.scale);
			draw_img(gl, &players.1, transform, Colors::SHADOW);
		},
	}
}

pub fn draw_player(c: Context, gl: &mut GlGraphics, map: &mut Map, cursor: &mut Cursor, players: (&GoElem, &GoElem))
{
	let mut near_pos: [f64; 2] = [0.0, 0.0];
	let board_x = GOBANPOS.0 + GOBAN_BOARD_X;
	let board_y = GOBANPOS.1 + GOBAN_BOARD_Y;
	let slot = find_slot_player!(map.current_player);

	for (y, pos_y) in map.value.iter().enumerate()
	{
		let new_posy = board_y + y as f64 * GOBAN_SPACE;
		for x in 0..19
		{

			let new_posx = board_x + x as f64 * GOBAN_SPACE;

			if cursor.press && 
				((new_posx - cursor.cursor_pos[0]).abs() + (new_posy - cursor.cursor_pos[1]).abs()) < 
				((near_pos[0] - cursor.cursor_pos[0]).abs() + (near_pos[1] - cursor.cursor_pos[1]).abs())
			{
				near_pos = [new_posx, new_posy];
				cursor.cursor_in_board = [x, y];
			}

			match (pos_y & (0o3 << (3 * (18 - x)))) >> 3 * (18 - x)
			{
				1 => {
						let transform = c.transform.trans(new_posx, new_posy).scale(players.0.scale, players.0.scale);
						draw_img(gl, &players.0, transform, Colors::NORMAL);
					},
				2 => {
						let transform = c.transform.trans(new_posx, new_posy).scale(players.1.scale, players.1.scale);
						draw_img(gl, &players.1, transform, Colors::NORMAL);
					},
				_ => {}
			}
		}
	}

	if cursor.press
	{
		draw_shadow(c, gl, players, near_pos, slot);
	}
}

pub fn draw_hint(c: Context, gl: &mut GlGraphics, map: &mut Map, players: (&GoElem, &GoElem), glyph_cache: &mut GlyphCache)
{
	let mut near_pos: [f64; 2] = [0.0, 0.0];
	let board_x = GOBANPOS.0 + GOBAN_BOARD_X;
	let board_y = GOBANPOS.1 + GOBAN_BOARD_Y;

	for (y, pos_y) in map.value.iter().enumerate()
	{
		let new_posy = board_y + y as f64 * GOBAN_SPACE;
		for x in 0..19
		{

			let new_posx = board_x + x as f64 * GOBAN_SPACE;
			if map.is_available((x, y as i128), &map.current_player) != 0
			{
				continue ;
			}
			let value = heuristic::value_slot(map, (y as i128, x as i128, 0), &map.current_player);

			match ((pos_y & (0o3 << (3 * (18 - x)))) >> 3 * (18 - x), value)
			{
				(0, 0) => {},
				(0, v) => {
						draw_text(gl, glyph_cache, &v.to_string(), c.transform.trans(new_posx, new_posy).scale(0.5, 0.5), Colors::BLACK);
					},
				_ => {}
			}
		}
	}
}