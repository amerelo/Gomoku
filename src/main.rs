#[macro_use]
extern crate gfx;
extern crate ggez;
extern crate rand;

// use rand::distributions::{IndependentSample, Range};
use ggez::{GameResult, Context};
use ggez::event::{self, MouseState, MouseButton};
use ggez::graphics::{self, DrawParam, Shader, Canvas, Point2, BlendMode, Drawable}; //DrawMode
use ggez::timer;
use ggez::conf;
use std::env;
use std::path;

gfx_defines!{
	/// Constants used by the shaders to calculate stuff
	constant Light
	{
		light_color: [f32; 4] = "u_LightColor",
		shadow_color: [f32; 4] = "u_ShadowColor",
		pos: [f32; 2] = "u_Pos",
		screen_size: [f32; 2] = "u_ScreenSize",
		glow: f32 = "u_Glow",
		strength: f32 = "u_Strength",
	}
}

struct WindowSettings {
	window_size_toggle: bool,
	toggle_fullscreen: bool,
	is_fullscreen: bool,
	num_of_resolutions: usize,
	resolution_index: usize,
	resize_projection: bool,
}


const OCCLUSIONS_SHADER_SOURCE: &[u8] = include_bytes!("../resources/occlusions_shader.glslv");
const VERTEX_SHADER_SOURCE: &[u8] = include_bytes!("../resources/basic_150.glslv");
const SHADOWS_SHADER_SOURCE: &[u8] = include_bytes!("../resources/shadows_shader.glslv");
const LIGHTS_SHADER_SOURCE: &[u8] = include_bytes!("../resources/lights_shader.glslv");

struct MainState {
	background: graphics::Image,
	tile: graphics::Image,
	goban: graphics::Image,
	text: graphics::Text,
	torch: Light,
	torch_p: Light,
	static_light: Light,
	foreground: Canvas,
	occlusions: Canvas,
	shadows: Canvas,
	lights: Canvas,
	occlusions_shader: Shader<Light>,
	shadows_shader: Shader<Light>,
	lights_shader: Shader<Light>,
	zoom: f32,
	window_settings: WindowSettings

}

/// The color cast things take when not illuminated
const AMBIENT_COLOR: [f32; 4] = [0.25, 0.22, 0.34, 1.0];
const AMBIENT_COLOR_P: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
/// The default color for the static light
const STATIC_LIGHT_COLOR: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
/// The default color for the mouse-controlled torch
const TORCH_COLOR: [f32; 4] = [0.24, 0.74, 0.54, 1.0];
/// The number of rays to cast to. Increasing this number will result in better
/// quality shadows. If you increase too much you might hit some GPU shader
/// hardware limits.
const LIGHT_RAY_COUNT: u32 = 1440;
/// The strength of the light - how far it shines
const LIGHT_STRENGTH: f32 = 0.0035;
/// The factor at which the light glows - just for fun
const LIGHT_GLOW_FACTOR: f32 = 0.0001;
/// The rate at which the glow effect oscillates
const LIGHT_GLOW_RATE: f32 = 50.0;

impl MainState 
{
	fn new(ctx: &mut Context) -> GameResult<MainState>
	{
		let background = graphics::Image::new(ctx, "/bg_top.png")?;

		let tile = graphics::Image::new(ctx, "/w_1.png")?; //  //tile.png //red_wood.jpg
		let goban = graphics::Image::new(ctx, "/goban.png")?; //  //tile.png //red_wood.jpg
		// let image1 = graphics::Image::new(ctx, "/dragon1.png")?;
		let text = {
			let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
			graphics::Text::new(ctx, "Gomoku...", &font)?
		};

		let screen_size = {
			let size = graphics::get_drawable_size(ctx);
			[size.0 as f32, size.1 as f32]
		};

		let torch = Light {
				pos: [0.0, 0.0],
				light_color: TORCH_COLOR,
				shadow_color: AMBIENT_COLOR,
				screen_size,
				glow: 0.0,
				strength: LIGHT_STRENGTH
		};

		let torch_p = Light {
				pos: [0.0, 0.0],
				light_color: TORCH_COLOR,
				shadow_color: AMBIENT_COLOR,
				screen_size,
				glow: 0.0,
				strength: LIGHT_STRENGTH
		};


		let (w, h) = graphics::get_size(ctx);
		let (x, y) = (100.0 / w as f32, 1.0 - 75.0 / h as f32);
		let static_light = Light
		{
			pos: [x, y],
			light_color: STATIC_LIGHT_COLOR,
			shadow_color: AMBIENT_COLOR,
			screen_size,
			glow: 0.0,
			strength: LIGHT_STRENGTH,
		};

		let foreground = Canvas::with_window_size(ctx)?;
		let occlusions = Canvas::new(ctx, LIGHT_RAY_COUNT, 1, conf::NumSamples::One)?;
		let mut shadows = Canvas::with_window_size(ctx)?;
		// The shadow map will be drawn on top using the multiply blend mode
		shadows.set_blend_mode(Some(BlendMode::Multiply));
		let mut lights = Canvas::with_window_size(ctx)?;
		// The light map will be drawn on top using the add blend mode
		lights.set_blend_mode(Some(BlendMode::Add));
		
		let occlusions_shader = Shader::from_u8(
			ctx,
			VERTEX_SHADER_SOURCE,
			OCCLUSIONS_SHADER_SOURCE,
			torch,
			"Light",
			None,
		).unwrap();
		
		let shadows_shader = Shader::from_u8(
			ctx,
			VERTEX_SHADER_SOURCE,
			SHADOWS_SHADER_SOURCE,
			torch,
			"Light",
			None,
		).unwrap();

		let lights_shader = Shader::from_u8(
			ctx,
			VERTEX_SHADER_SOURCE,
			LIGHTS_SHADER_SOURCE,
			torch,
			"Light",
			Some(&[BlendMode::Add]),
		).unwrap();
		let zoom = 1.0;

		let window_settings =  WindowSettings {
			toggle_fullscreen: false,
			window_size_toggle: false,
			is_fullscreen: false,
			resolution_index: 0,
			num_of_resolutions: 0,
			resize_projection: false,
		};

		Ok(MainState {
			background, tile, goban, text, torch, torch_p, static_light,
			foreground, occlusions, shadows, lights, occlusions_shader, shadows_shader,
			lights_shader, zoom, window_settings})
	}

	fn render_light( &mut self, ctx: &mut Context, light: Light, origin: DrawParam, canvas_origin: DrawParam ) -> GameResult<()> 
	{
		let size = graphics::get_size(ctx);
		// Now we want to run the occlusions shader to calculate our 1D shadow
		// distances into the `occlusions` canvas.
		graphics::set_canvas(ctx, Some(&self.occlusions));
		{
			let _shader_lock = graphics::use_shader(ctx, &self.occlusions_shader);

			self.occlusions_shader.send(ctx, light)?;
			graphics::draw_ex(ctx, &self.foreground, canvas_origin)?;
		}

		// Now we render our shadow map and light map into their respective
		// canvases based on the occlusion map. These will then be drawn onto
		// the final render target using appropriate blending modes.
		graphics::set_canvas(ctx, Some(&self.shadows));
		{
			let _shader_lock = graphics::use_shader(ctx, &self.shadows_shader);

			let param = DrawParam {
				scale: Point2::new((size.0 as f32) / (LIGHT_RAY_COUNT as f32), size.1 as f32),
				..origin
			};
			self.shadows_shader.send(ctx, light)?;
			graphics::draw_ex(ctx, &self.occlusions, param)?;
		}
		graphics::set_canvas(ctx, Some(&self.lights));
		{
			let _shader_lock = graphics::use_shader(ctx, &self.lights_shader);

			let param = DrawParam {
				scale: Point2::new((size.0 as f32) / (LIGHT_RAY_COUNT as f32), size.1 as f32),
				..origin
			};
			self.lights_shader.send(ctx, light)?;
			graphics::draw_ex(ctx, &self.occlusions, param)?;
		}
		Ok(())
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()>
	{	
		// if timer::get_ticks(ctx) % 100 == 0
		// {
		// 	println!("Average FPS: {}", timer::get_fps(ctx));
		// }
		// let i = Range::new(1, 5);
		
		// let mut rng = rand::thread_rng();
		// let choice = i.ind_sample(&mut rng);
		// println!("{}", choice);

		// let new_str = String::from("/dragon") + &choice.to_string() + ".png";
		// self.tile = graphics::Image::new(ctx, new_str)?;

		self.torch.glow =
			LIGHT_GLOW_FACTOR * ((timer::get_ticks(ctx) as f32) / LIGHT_GLOW_RATE).cos();
		self.static_light.glow =
			LIGHT_GLOW_FACTOR * ((timer::get_ticks(ctx) as f32) / LIGHT_GLOW_RATE * 0.75).sin();
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()>
	{
		let size = graphics::get_size(ctx);

		let origin = DrawParam {
			dest: Point2::new(0.0, 0.0),
			..Default::default()
		};

		let origin_deux = DrawParam {
			dest: Point2::new(70.0, 0.0),
			scale: Point2::new(1.4, 1.4),
			..Default::default()
		};

		// for re-rendering canvases, we need to take the DPI into account
		let dpiscale = {
			let dsize = graphics::get_drawable_size(ctx);
			Point2::new(
				size.0 as f32 / dsize.0 as f32,
				size.1 as f32 / dsize.1 as f32
			)
		};

		let canvas_origin = DrawParam
		{
			scale: dpiscale,
			..origin
		};

		// First thing we want to do it to render all the foreground items (that
		// will have shadows) onto their own Canvas (off-screen render). We will
		// use this canvas to:
		//  - run the occlusions shader to determine where the shadows are
		//  - render to screen once all the shadows are calculated and rendered
		graphics::set_canvas(ctx, Some(&self.foreground));
		graphics::set_background_color(ctx, [0.0; 4].into());
		graphics::clear(ctx);


		let y_start: f32 = 75.0;
		let y_dif: f32 = 32.2;

		for elem in 0..19 {
			let new_pos = y_start + (y_dif * elem as f32);

			graphics::draw_ex(
				ctx,
				&self.tile,
				DrawParam {
					dest: Point2::new(new_pos, 133.0),
					scale: Point2::new(0.10, 0.10),
					..Default::default()
				},
			)?;
		}

		// graphics::draw_ex(
		//     ctx,
		//     &self.tile,
		//     DrawParam {
		//         dest: Point2::new(442.0, 468.0),
		//         rotation: 0.5,
		//         ..Default::default()
		//     },
		// )?;

		// graphics::draw(ctx, &self.text, Point2::new(50.0, 200.0), 0.0)?;

		// First we draw our light and shadow maps
		let torch = self.torch;
		let torch_p = self.torch_p;
		let light = self.static_light;

		graphics::set_canvas(ctx, Some(&self.lights));
		graphics::clear(ctx);
		graphics::set_canvas(ctx, Some(&self.shadows));
		graphics::clear(ctx);

		self.render_light(ctx, torch, origin, canvas_origin)?;
		self.render_light(ctx, torch_p, origin, canvas_origin)?;

		// Now lets finally render to screen starting with out background, then
		// the shadows and lights overtop and finally our foreground.
		graphics::set_canvas(ctx, None);
		graphics::set_color(ctx, graphics::WHITE)?;

		graphics::draw_ex(ctx, &self.background, origin)?;
		graphics::draw_ex(ctx, &self.goban, origin_deux)?;
		graphics::draw_ex(ctx, &self.shadows, origin)?;
		graphics::draw_ex(ctx, &self.lights, origin)?;

		// We switch the color to the shadow color before drawing the foreground objects
		// this has the same effect as applying this color in a multiply blend mode with
		// full opacity. We also reset the blend mode back to the default Alpha blend mode.
		graphics::set_color(ctx, AMBIENT_COLOR_P.into())?;
		graphics::draw_ex(ctx, &self.foreground, origin)?;

		graphics::present(ctx);
		Ok(())
	}

	fn mouse_motion_event(&mut self, ctx: &mut Context, _state: MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32)
	{
		let (w, h) = graphics::get_size(ctx);
		let (x, y) = (x as f32 / w as f32, 1.0 - y as f32 / h as f32);
		self.torch.pos = [x, y];
	}

	fn mouse_button_down_event(&mut self, ctx: &mut Context, _state: MouseButton, x: i32, y: i32)
	{
		let (w, h) = graphics::get_size(ctx);
		println!("{:#?}", [x, y]);
		let (x, y) = (x as f32 / w as f32, 1.0 - y as f32 / h as f32);
		
		self.torch_p.pos = [x, y];
	}

	fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
		println!("Resized screen to {}, {}", width, height);
		if self.window_settings.resize_projection {
			let new_rect = graphics::Rect::new(
				0.0,
				0.0,
				width as f32 * self.zoom,
				height as f32 * self.zoom,
			);
			graphics::set_screen_coordinates(ctx, new_rect).unwrap();
		}
	}
}

pub fn main()
{
	let mut c = conf::Conf::new();
	c.window_mode.fullscreen_type = conf::FullscreenType::Off;
	c.window_setup.resizable = true;

	let ctx = &mut Context::load_from_conf("shadows", "ggez", c).unwrap();

	if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR")
	{
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("resources");
		ctx.filesystem.mount(&path, true);
	}

	let state = &mut MainState::new(ctx).unwrap();
	event::run(ctx, state).unwrap();
}

