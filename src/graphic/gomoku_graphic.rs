use piston::window::WindowSettings;

use std::time::{Instant, Duration};
// use sdl2_window::Sdl2Window as Window;
use piston_window::*;
use sdl2_window::Sdl2Window;
// use piston_window::PistonWindow as Window;

// use piston::event_loop::*;
// use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
use find_folder::Search;

use fps_counter::FPSCounter;
use goban::player::{Player};
use goban::map::{Map};
// use graphics::*;
use graphic::loader::{ GoElem };
use graphic::cursor::{ Cursor };
use graphic::draw::{ draw_goban, draw_player, draw_text };
use minmax::recursive::{ start_min_max };
use heuristic;

const BACKGROUND:[f32; 4] = [0.65, 0.55, 0.45, 1.0];
// 0.95, 0.69, 0.50

pub struct App {
	fps: FPSCounter,
	gl: GlGraphics, // OpenGL drawing backend.
	goban: GoElem,
	go_w: GoElem,
	go_b: GoElem,
	map: Map,
	cursor: Cursor,
	my_time: f64,
}

impl App
{
	fn new(opengl: OpenGL) -> Self
	{
		App {
			fps: FPSCounter::new(),
			gl: GlGraphics::new(opengl),
			map: Map {..Default::default() },
			goban: GoElem::new("resources/goban.png", 1.5),
			go_b: GoElem::new("resources/w_1.png", 0.09),
			go_w: GoElem::new("resources/black.png", 0.10),
			cursor: Cursor::new(),
			my_time: 0.0,
		}
	}

	fn render(&mut self, args: &RenderArgs, mut glyph_cache: &mut GlyphCache) //RenderArgs
	{
		let goban = &self.goban;
		let map = &mut self.map;
		let players = (&self.go_w, &self.go_b);
		let mut tmp_cursor = &mut self.cursor;

		let fps_t = &format!("fps: {}            time of last AI move: {:.5} ms", self.fps.tick(), self.my_time);
		// let turn = ;

		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);

			draw_text(c, gl, &mut glyph_cache, fps_t, c.transform.trans(5.0, 20.0));
			draw_text(c, gl, &mut glyph_cache, &format!("Turn: {}", map.turn), c.transform.trans(5.0, 40.0));
			draw_goban(c, gl, goban);
			draw_player(c, gl, map, &mut tmp_cursor, players);
		});

		// let player_turn = find_slot_player!(map.current_player, Slot::PlayerOne, Slot::PlayerTwo);
		if map.current_player == Player::Two
		{
			let now = Instant::now();
			match start_min_max(&map)
			{
				Some(action) => {
					map.set_value((action.x_y.0 as i128, action.x_y.1 as i128), find_slot_player!(map.current_player));
					// map.number_captured((action.x_y.0 as i32, action.x_y.1 as i32), (&Slot::PlayerTwo, &Slot::PlayerOne), true);
					// map.change_player_turn();
				},
				None => (),
			}
			map.change_player_turn();
			let elapsed = now.elapsed();
			let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
			self.my_time = sec;
		} 
		else if !tmp_cursor.press && tmp_cursor.place_piece &&
			map.is_available((tmp_cursor.cursor_in_board[0] as i128, tmp_cursor.cursor_in_board[1] as i128)) == 0
		{
			// let slot_player = &find_slot_player![map.current_player, Slot::PlayerOne, Slot::PlayerTwo];
			// let slot_enemy = &find_slot_enemy![map.current_player, Slot::PlayerOne, Slot::PlayerTwo];

			// map.is_winning_move((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32));
			
			// map.number_captured((tmp_cursor.cursor_in_board[0] as i32, tmp_cursor.cursor_in_board[1] as i32), (slot_player, slot_enemy), true);
			
			map.set_value((tmp_cursor.cursor_in_board[0] as i128, tmp_cursor.cursor_in_board[1] as i128), find_slot_player!(map.current_player));
			map.change_player_turn();

			// println!("player one {}\nplayer two {}\n", heuristic::map_value(map, (&Slot::PlayerOne, &Slot::PlayerTwo)), heuristic::map_value(map, (&Slot::PlayerTwo, &Slot::PlayerOne)));
			
			tmp_cursor.place_piece = false;
		}
	}

	// fn update(&mut self, _args: &UpdateArgs)
	// {
		// println!("fps => {}", self.fps.tick());
		// println!("time => {}", args.dt);
		// Rotate 2 radians per second.
		// self.rotation += 2.0 * args.dt;
	// }
}

pub fn start()
{
	let opengl = OpenGL::V3_2;

	let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(
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
	// .max_fps(200)
	// .lazy(true)

	let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let ref font = assets.join("DejaVuSerif.ttf");

	let mut glyph_cache = GlyphCache::new(
	font,
	(),
	TextureSettings::new(),
	).unwrap();

	// let text = Text::new(20);

	window.set_lazy(true);
	while let Some(e) = events.next(&mut window)
	{
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

		if let Some(r) = e.render_args()
		{
			// println!("fps => {}", app.fps.tick());
			app.render(&r, &mut glyph_cache);
			// draw_text(&e, &mut window, &mut app, &glyphs);
			// draw_hint(&e, &mut window, &mut app);
		}

		// if let Some(u) = e.update_args()
		// {
		// 	println!("--------- {:?}", u);
		// 	app.update(&u);
		// }
	}
	
	// fn draw_hint(e: &Event, window: &mut PistonWindow<Sdl2Window>, app: &mut App)
	// {
	// 	let assets = Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	// 	let ref font = assets.join("DejaVuSerif.ttf");
	// 	let factory = window.factory.clone();
	// 	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

	// 	const GOBAN_SPACE: f64 = 34.5;
	// 	const GOBANPOS: (f64, f64) = (70.0, 40.0);
	// 	const GOBAN_BOARD_X: f64 = 8.0;
	// 	const GOBAN_BOARD_Y: f64 = 10.0;
	// 	let board_x = GOBANPOS.0 + GOBAN_BOARD_X;
	// 	let board_y = GOBANPOS.1 + GOBAN_BOARD_Y;
	// 	let mut map = &mut app.map.clone();
	// 	let mut map2 = app.map.clone();
	// 	let slot_player = find_slot_player![map.current_player];

	// 	for (y, pos_y) in map.value.iter().enumerate()
	// 	{
	// 		let new_posy = board_y + y as f64 * GOBAN_SPACE;
	// 		for (x, pos_x) in pos_y.iter().enumerate()
	// 		{
	// 			let new_posx = board_x + x as f64 * GOBAN_SPACE;

	// 			if  Slot::Empty == *pos_x
	// 			{
	// 				map2.value[y][x] = slot_player;
	// 				window.draw_2d(e, |c, gl| {
	// 					let transform = c.transform.trans(new_posx, new_posy);
	// 					let _ = Text::new_color([0.0, 0.0, 0.0, 1.0], 10).draw(
	// 						&format!("{}", heuristic::map_value(&map2, find_slots_players![map.current_player])),
	// 						&mut glyphs,
	// 						&c.draw_state,
	// 						transform, gl
	// 					);
	// 				});
	// 				map2.value[y][x] = Slot::Empty;
	// 			}
	// 		}
	// 	}
	// }
}