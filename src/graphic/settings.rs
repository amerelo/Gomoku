// use fps_counter::FPSCounter;

use piston::window::WindowSettings;
use piston_window::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
// use graphic::loader::{ GoElem };
// use graphic::cursor::{ Cursor };
use graphic::draw::{ draw_goban, draw_player, draw_text };

const BACKGROUND:[f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Settings {
	gl: GlGraphics,
}

impl Settings
{
	pub fn new(opengl: OpenGL) -> Self
	{
		Settings {
			gl: GlGraphics::new(opengl),
		}
	}

	pub fn render(&mut self, args: &RenderArgs, mut glyph_cache: &mut GlyphCache) //RenderArgs
	{
		self.gl.draw(args.viewport(), |c, gl|
		{
			clear(BACKGROUND, gl);

			draw_text(c, gl, &mut glyph_cache, &format!(" test "), c.transform.trans(200.0, 200.0));
			// draw_text(c, gl, &mut glyph_cache, &format!("Turn: {}", map.turn), c.transform.trans(5.0, 40.0));
		});
	}
}