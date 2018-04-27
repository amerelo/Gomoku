
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;
use piston::event_loop::*;
use piston::input::*;

// use fps_counter::FPSCounter;

use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };

use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor };
use goban::map::{Map, Slot};
// use goban::player::{Player, PlayerKind};

pub struct App {
	// fps: FPSCounter,
	gl: GlGraphics, // OpenGL drawing backend.
	
	goban: GoElem,
	go_w: GoElem,
	go_b: GoElem,
	
	map: Map,
	cursor: Cursor,
	// cursor_pos: [f64; 2],
	// board_pos: [i32; 2],
	// press: bool,
}

const BACKGROUND:[f32; 4] = [0.2, 0.2, 0.2, 1.0];
const GOBANPOS: (f64, f64) = (70.0, 0.0);
const GOBAN_BOARD_X: f64 = 8.0;
const GOBAN_BOARD_Y: f64 = 10.0;
const GOBAN_SPACE: f64 = 34.5;

impl App
{
	fn new(opengl: OpenGL) -> Self
	{
		App {
			// fps: FPSCounter::new(),
			gl: GlGraphics::new(opengl),
			map: Map {..Default::default() },
			
			goban: GoElem::new("resources/goban.png", 1.5),
			go_w: GoElem::new("resources/w_1.png", 0.09),
			go_b: GoElem::new("resources/black.png", 0.09),
			
			cursor: Cursor::new(),
			// cursor_pos: [0.0, 0.0],
			// board_pos: [-1, -1],
			// press: false,
		}
	}
	
	fn render(&mut self, args: &RenderArgs)
	{
		let goban = &self.goban;
		let go_w = &self.go_w;
		let map = &mut self.map;
		let mut tmp_cursor = &mut self.cursor;

		// println!("fps => {}", self.fps.tick());
		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);
			draw_goban(c, gl, goban);
			draw_w(c, gl, go_w, map, &mut tmp_cursor);
		});

		if !tmp_cursor.press && tmp_cursor.place_piece &&
			map.is_available((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32)) == Slot::Empty
		{
			map.value[tmp_cursor.cursor_in_board[1]][tmp_cursor.cursor_in_board[0]] = Slot::PlayerOne;
			tmp_cursor.place_piece = false;
		}
	}

	fn update(&mut self, _args: &UpdateArgs)
	{
		// println!("time => {}", args.dt);
		// Rotate 2 radians per second.
		// self.rotation += 2.0 * args.dt;
	}
}

fn draw_goban(c: Context, gl: &mut GlGraphics, goban: &GoElem)
{
	let (newx, newy) = (GOBANPOS.0, GOBANPOS.1);

	let transform2 = c.transform.trans(newx, newy).scale(goban.scale, goban.scale);
	image(&goban.elem, transform2, gl);
}

fn draw_w(c: Context, gl: &mut GlGraphics, go_player: &GoElem, map: &Map, cursor: &mut Cursor)
{
	let mut near_pos: [f64; 2] = [0.0, 0.0];
	let board_x = GOBANPOS.0 + GOBAN_BOARD_X;

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
				let transform = c.transform.trans(new_posx, new_posy).scale(go_player.scale, go_player.scale);
				image(&go_player.elem, transform, gl);
			}
		}
	}
	
	if cursor.press && (near_pos[0] != 0.0 && near_pos[1] != 0.0)
	{
		let transform = c.transform.trans(near_pos[0], near_pos[1]).scale(go_player.scale, go_player.scale);
		Image::new_color([1.0, 1.0, 1.0, 0.6]).draw(&go_player.elem, &DrawState::new_alpha(), transform, gl);
	}
}

pub fn start()
{
	let opengl = OpenGL::V3_2;

	// Create an Glutin window.
	let mut window: Window = WindowSettings::new(
			"Gomoku",
			[800, 700]
		)
		.resizable(false)
		.opengl(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut app = App::new(opengl);
	let mut events = Events::new(EventSettings::new());
	
	while let Some(e) = events.next(&mut window)
	{
		if let Some(r) = e.render_args()
		{
			app.render(&r);
		}

		if let Some(u) = e.update_args()
		{
			app.update(&u);
		}

		if let Some(button) = e.press_args()
		{
			if button == Button::Mouse(MouseButton::Left)
			{
				// println!("press {:?}", button);
				app.cursor.place_piece = false;
				app.cursor.press = true;
			}
		}
		
		if let Some(button) = e.release_args()
		{
			if button == Button::Mouse(MouseButton::Left)
			{
				// println!("release {:?}", button);
				app.cursor.place_piece = true;
				app.cursor.press = false;
			}
		}

		if let Some(pos) = e.mouse_cursor_args()
		{
			app.cursor.cursor_pos = pos;
			// if app.press 
			// {
			// 	println!("pos mouse -> {:?}", pos);
			// }
		}
	}
}