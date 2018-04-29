use graphics::*;

use opengl_graphics::{ GlGraphics };
use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor };
use goban::map::{Map, Slot};
use goban::player::{Player};

const GOBANPOS: (f64, f64) = (70.0, 0.0);
const GOBAN_BOARD_X: f64 = 8.0;
const GOBAN_BOARD_Y: f64 = 10.0;
const GOBAN_SPACE: f64 = 34.5;

pub fn draw_goban(c: Context, gl: &mut GlGraphics, goban: &GoElem)
{
	let (newx, newy) = (GOBANPOS.0, GOBANPOS.1);

	let transform2 = c.transform.trans(newx, newy).scale(goban.scale, goban.scale);
	image(&goban.elem, transform2, gl);
}

fn draw_shadow(c: Context, gl: &mut GlGraphics, cursor: &mut Cursor, players: (&GoElem, &GoElem), near_pos: [f64; 2], slot: Slot)
{
	if cursor.press && (near_pos[0] != 0.0 && near_pos[1] != 0.0)
	{
		match slot
		{
			Slot::PlayerOne => {
				let transform = c.transform.trans(near_pos[0], near_pos[1]).scale(players.0.scale, players.0.scale);
				Image::new_color([1.0, 1.0, 1.0, 0.6]).draw(&players.0.elem, &DrawState::new_alpha(), transform, gl);
			},
			_ 				=> {
				let transform = c.transform.trans(near_pos[0], near_pos[1]).scale(players.1.scale, players.1.scale);
				Image::new_color([1.0, 1.0, 1.0, 0.6]).draw(&players.1.elem, &DrawState::new_alpha(), transform, gl);
			},
		}
	}
}

pub fn draw_player(c: Context, gl: &mut GlGraphics, map: &mut Map, cursor: &mut Cursor, players: (&GoElem, &GoElem))
{
	let mut near_pos: [f64; 2] = [0.0, 0.0];
	let board_x = GOBANPOS.0 + GOBAN_BOARD_X;
	let slot =  find_slot_player!(map.current_player, Slot::PlayerOne, Slot::PlayerTwo);

	for (y, pos_y) in map.value.iter().enumerate()
	{
		let new_posy = GOBAN_BOARD_Y + y as f64 * GOBAN_SPACE;
		for (x, pos_x) in pos_y.iter().enumerate()
		{
			// near_pos
			let new_posx = board_x + x as f64 * GOBAN_SPACE;
			if cursor.press && 
				((new_posx - cursor.cursor_pos[0]).abs() + (new_posy - cursor.cursor_pos[1]).abs()) < 
				((near_pos[0] - cursor.cursor_pos[0]).abs() + (near_pos[1] - cursor.cursor_pos[1]).abs())
			{
				near_pos = [new_posx, new_posy];
				cursor.cursor_in_board = [x, y];
			}

			if  Slot::Empty != *pos_x
			{
				match *pos_x
				{
					Slot::PlayerOne => {
						let transform = c.transform.trans(new_posx, new_posy).scale(players.0.scale, players.0.scale);
						image(&players.0.elem, transform, gl);
					},
					_ 				=> {
						let transform = c.transform.trans(new_posx, new_posy).scale(players.1.scale, players.1.scale);
						image(&players.1.elem, transform, gl);
					},
				}
			}
		}
	}
	draw_shadow(c, gl, cursor, players, near_pos, slot);
}
