#[macro_use]
extern crate gfx;
extern crate ggez;
extern crate rand;

mod mainstate;

use mainstate::{MainState};

use ggez::{Context, conf, event};
use std::{env, path};

// const OCCLUSIONS_SHADER_SOURCE: &[u8] = include_bytes!("../resources/occlusions_shader.glslv");
// const VERTEX_SHADER_SOURCE: &[u8] = include_bytes!("../resources/basic_150.glslv");
// const SHADOWS_SHADER_SOURCE: &[u8] = include_bytes!("../resources/shadows_shader.glslv");
// const LIGHTS_SHADER_SOURCE: &[u8] = include_bytes!("../resources/lights_shader.glslv");



// /// The color cast things take when not illuminated
// const AMBIENT_COLOR: [f32; 4] = [0.25, 0.22, 0.34, 1.0];
// const AMBIENT_COLOR_P: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
// /// The default color for the static light
// const STATIC_LIGHT_COLOR: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
// /// The default color for the mouse-controlled torch
// const TORCH_COLOR: [f32; 4] = [0.24, 0.74, 0.54, 1.0];

// /// The number of rays to cast to. Increasing this number will result in better
// /// quality shadows. If you increase too much you might hit some GPU shader
// /// hardware limits.
// const LIGHT_RAY_COUNT: u32 = 1440;
// /// The strength of the light - how far it shines
// const LIGHT_STRENGTH: f32 = 0.00035;
// /// The factor at which the light glows - just for fun
// const LIGHT_GLOW_FACTOR: f32 = 0.0000001;
// /// The rate at which the glow effect oscillates
// const LIGHT_GLOW_RATE: f32 = 50.0;


pub fn main()
{
	let mut c = conf::Conf::new();
	c.window_mode.width = 1500;
	c.window_mode.height = 1200;
	c.window_mode.fullscreen_type = conf::FullscreenType::Off;
	c.window_setup.resizable = true;
	c.window_setup.title = String::from("Gomoku");

	let ctx = &mut Context::load_from_conf("shadows", "amerelo-ocarta", c).unwrap();

	if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR")
	{
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("resources");
		ctx.filesystem.mount(&path, true);
	}

	let state = &mut MainState::new(ctx).unwrap();
	event::run(ctx, state).unwrap();
}

