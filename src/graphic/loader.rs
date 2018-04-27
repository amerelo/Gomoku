
use opengl_graphics::{ Texture, TextureSettings };
use std::path::Path;

pub struct GoElem
{
	pub elem: Texture,
	pub scale: f64,
}

impl GoElem 
{
	pub fn new (path: &str, scale: f64) -> Self
	{
		GoElem {
			elem: Texture::from_path(Path::new(path), &TextureSettings::new()).unwrap(),
			scale: scale,
		}
	}
}