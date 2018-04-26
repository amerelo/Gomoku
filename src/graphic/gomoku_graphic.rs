
use std::path::Path;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;

use fps_counter::FPSCounter;

use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL, Texture, TextureSettings };
use glutin_window::GlutinWindow as Window;

pub struct App {
	fps: FPSCounter,
	gl: GlGraphics, // OpenGL drawing backend.
	rotation: f64,   // Rotation for the square.
	dragon: Texture
}

impl App
{
	pub fn render(&mut self, args: &RenderArgs)
	{
		const GREEN:[f32; 4] = [0.6, 0.4, 1.0, 1.0];
		const RED:	[f32; 4] = [0.4, 0.5, 0.4, 1.0];

		let square = rectangle::square(0.0, 0.0, 50.0);
		let rotation = self.rotation;
		let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
		let dragon = &self.dragon;

		// println!("fps => {}", self.fps.tick());	

		self.gl.draw(args.viewport(), |c, gl|
		{
			// Clear the screen.
			clear(GREEN, gl);

			let transform = c.transform.trans(x, y)
										.rot_rad(rotation)
										.trans(-25.0, -25.0);

			let transform2 = c.transform.trans(10.0, 10.0)
										.scale(0.5, 0.5);

			image(dragon, transform2, gl);
			// Draw a box rotating around the middle of the screen.
			rectangle(RED, square, transform, gl);
		});
	}

	pub fn update(&mut self, args: &UpdateArgs)
	{
		// println!("time => {}", args.dt);
		
		// Rotate 2 radians per second.
		self.rotation += 2.0 * args.dt;
	}
}

pub fn start()
{
	// Change this to OpenGL::V2_1 if not working.
	let opengl = OpenGL::V3_2;

	// Create an Glutin window.
	let mut window: Window = WindowSettings::new(
			"spinning-square",
			[1000, 1000]
		)
		.opengl(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	// Create a new game and run it.
	let mut app = App
	{
		fps: FPSCounter::new(),
		gl: GlGraphics::new(opengl),
		rotation: 0.0,
		dragon: Texture::from_path(Path::new("resources/dragon4.png"), &TextureSettings::new()).unwrap()
	};

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
	}
}