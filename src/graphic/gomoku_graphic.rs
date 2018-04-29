use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };

// use fps_counter::FPSCounter;
use goban::player::{Player};
use goban::map::{Map, Slot};
use graphics::*;
use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor };
use graphic::draw::{ draw_goban, draw_player };

const BACKGROUND:[f32; 4] = [0.2, 0.2, 0.2, 1.0];

pub struct App {
	// fps: FPSCounter,
	gl: GlGraphics, // OpenGL drawing backend.
	goban: GoElem,
	go_w: GoElem,
	go_b: GoElem,
	map: Map,
	cursor: Cursor,
}

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
			go_b: GoElem::new("resources/black.png", 0.10),
			cursor: Cursor::new(),
		}
	}
	
	fn render(&mut self, args: &RenderArgs)
	{
		let goban = &self.goban;
		let map = &mut self.map;
		let players = (&self.go_w, &self.go_b);
		let mut tmp_cursor = &mut self.cursor;

		// println!("fps => {}", self.fps.tick());
		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);
			draw_goban(c, gl, goban);
			draw_player(c, gl, map, &mut tmp_cursor, players)
		});

		if !tmp_cursor.press && tmp_cursor.place_piece &&
			map.is_available((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32)) == Slot::Empty
		{
			map.is_winning_move((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32));
			map.number_captured((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32), true);
			map.value[tmp_cursor.cursor_in_board[1]][tmp_cursor.cursor_in_board[0]] = find_slot_player!(map.current_player);//map.get_palyer_slot();
			map.change_player_turn();
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