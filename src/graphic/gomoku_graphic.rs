use piston::window::WindowSettings;

// use sdl2_window::Sdl2Window as Window;
use piston_window::*;
use sdl2_window::Sdl2Window;
// use piston_window::PistonWindow as Window;

// use piston::event_loop::*;
// use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };

// use fps_counter::FPSCounter;
use goban::player::{Player};
use goban::map::{Map, Slot, HintSlot};
use graphics::*;
use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor };
use graphic::draw::{ draw_goban, draw_player };

use find_folder::Search;

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
			draw_player(c, gl, map, &mut tmp_cursor, players);
		});

		if !tmp_cursor.press && tmp_cursor.place_piece &&
			map.is_available((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32)) == Slot::Empty
		{
			map.is_winning_move((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32));
			map.number_captured((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32), true);
			map.value[tmp_cursor.cursor_in_board[1]][tmp_cursor.cursor_in_board[0]] = find_slot_player!(map.current_player, Slot::PlayerOne, Slot::PlayerTwo);//map.get_palyer_slot();
			map.hint_map[tmp_cursor.cursor_in_board[1]][tmp_cursor.cursor_in_board[0]] = HintSlot::Used;
			map.change_player_turn();
			map.update_hint_map((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32));
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

	let mut window: PistonWindow<Sdl2Window>= WindowSettings::new(
			"Gomoku",
			[800, 700]
		)
		.opengl(opengl)
		.resizable(false)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut app = App::new(opengl);
	let mut events = Events::new(EventSettings::new());


	let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let ref font = assets.join("DejaVuSerif.ttf");
	let factory = window.factory.clone();
	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

	window.set_lazy(true);
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

		window.draw_2d(&e, |c, gl| {
			let transform = c.transform.trans(10.0, 100.0);

			let _ = text::Text::new_color([0.4, 0.4, 0.4, 1.0], 32).draw(
				"Hola Alexis",
				&mut glyphs,
				&c.draw_state,
				transform, gl
			);
		});
	}
}