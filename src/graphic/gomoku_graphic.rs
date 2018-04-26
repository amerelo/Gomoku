use std::path::Path;

use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;
use piston::event_loop::*;
use piston::input::*;

use fps_counter::FPSCounter;

use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL, Texture, TextureSettings };

use gomoku::map::{Map, Slot};
use gomoku::player::{Player, PlayerKind};
// use glutin_window::GlutinWindow as Window;

pub struct App {
	fps: FPSCounter,
	gl: GlGraphics, // OpenGL drawing backend.
	goban: Texture,
	go_w: Texture,
	map: Map,
	cursor_pos: [f64; 2],
	board_pos: [usize; 2],
	press: bool,
}

const BACKGROUND:[f32; 4] = [0.2, 0.2, 0.2, 1.0];
const GOBANPOS:(f64, f64) = (70.0, 0.0);

impl App
{
	fn new(opengl: OpenGL) -> Self
	{
		App {
			fps: FPSCounter::new(),
			gl: GlGraphics::new(opengl),
			goban: Texture::from_path(Path::new("resources/goban.png"), &TextureSettings::new()).unwrap(),
			// gonban_c: Context.transform.trans.trans(newx, newy).scale(1.5, 1.5);
			go_w: Texture::from_path(Path::new("resources/w_1.png"), &TextureSettings::new()).unwrap(),
			map: Map {..Default::default() },
			cursor_pos: [0.0, 0.0],
			board_pos: [0, 0],
			press: false,
		}
	}
	
	fn render(&mut self, args: &RenderArgs)
	{
		let goban = &self.goban;
		let go_w = &self.go_w;
		let newx = self.cursor_pos[0];
		let newy = self.cursor_pos[1];
		let map = &mut self.map;
		let press = self.press;
		let mut board_pos = [self.board_pos[0], self.board_pos[1]];

		// println!("fps => {}", self.fps.tick());
		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);

			draw_goban(c, gl, goban);
			draw_w(c, gl, go_w, map, [newx, newy], &mut board_pos, press);
		});

		if !press && board_pos[0] != 0 && board_pos[1] != 0
		{
			map.value[board_pos[1]][board_pos[0]] = Slot::PlayerTwo;
		}
		self.board_pos[0] = board_pos[0];
		self.board_pos[1] = board_pos[1];
	}

	fn update(&mut self, args: &UpdateArgs)
	{
		// println!("time => {}", args.dt);
		
		// Rotate 2 radians per second.
		// self.rotation += 2.0 * args.dt;
	}
}

fn draw_goban(c: Context, gl: &mut GlGraphics, goban: &Texture)
{
	let (newx, newy) = (GOBANPOS.0, GOBANPOS.1);

	let transform2 = c.transform.trans(newx, newy).scale(1.5, 1.5);
	image(goban, transform2, gl);
}

fn draw_w(c: Context, gl: &mut GlGraphics, go_w: &Texture, map: &Map, cursor_pos: [f64; 2], board_pos: &mut [usize; 2], press: bool)
{

	let mut near_pos: [f64; 2] = [0.0, 0.0];

	let space_x = GOBANPOS.0 + 8.0;
	let space_y = 10.0;
	let space_c = 34.5;

	for (y, pos_y) in map.value.iter().enumerate()
	{
		let new_posy = space_y + y as f64 * space_c;
		for (x, pos_x) in pos_y.iter().enumerate()
		{
			// near_pos
			let new_posx = space_x + x as f64 * space_c;
			if press && ( (new_posx - cursor_pos[0]).abs() + (new_posy - cursor_pos[1]).abs()) < ((near_pos[0] - cursor_pos[0]).abs() + (near_pos[1] - cursor_pos[1]).abs())
			{
				near_pos[0] = new_posx;
				near_pos[1] = new_posy;
				*board_pos = [x, y];
			}
			// println!("{}", (near_pos[0] - cursor_pos[0]).abs() + (near_pos[1] - cursor_pos[1]).abs() );

			if  Slot::Empty != *pos_x
			{
				let transform = c.transform.trans(new_posx, new_posy)
											.scale(0.09, 0.09);
				image(go_w, transform, gl);
			}
		}
	}
	
	if press && (near_pos[0] != 0.0 && near_pos[1] != 0.0)
	{
		let transform = c.transform.trans(near_pos[0], near_pos[1])
							.scale(0.09, 0.09);
		Image::new_color([1.0, 1.0, 1.0, 0.6]).draw(go_w, &DrawState::new_alpha(), transform, gl);
	}
}

// fn near_cursor(c: Context, gl: &mut GlGraphics, go_w: &Texture, cursor_pos: [f64; 2] , map: &Map)
// {
	// let transform = c.transform.trans(space_x + x as f64 * space_c , space_y + y as f64 * space_c)
	// 						.scale(0.09, 0.09);
	// image(go_w, transform, gl);
// }

pub fn start()
{
	let opengl = OpenGL::V3_2;

	// Create an Glutin window.
	let mut window: Window = WindowSettings::new(
			"Gomoku",
			[800, 700]
		)
		// .resizable(false)
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
				println!("press {:?}", button);
				app.board_pos = [0, 0];
				app.press = true;
			}
		}
		
		if let Some(button) = e.release_args()
		{
			if button == Button::Mouse(MouseButton::Left)
			{
				println!("release {:?}", button);
				app.press = false;
			}
		}

		if let Some(pos) = e.mouse_cursor_args()
		{
			app.cursor_pos = pos;
			
			if app.press 
			{
				println!("pos mouse -> {:?}", pos);
			}
		}
	}
}