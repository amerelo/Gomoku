use piston::window::WindowSettings;

use piston_window::*;
use sdl2_window::Sdl2Window;

use opengl_graphics::{ OpenGL, GlyphCache };
use find_folder::Search;
use goban::map::{ Map };

use graphic::cursor::{ Cursor, Scene, Controls};
use graphic::settings::{ Settings };
use graphic::{ game::{ Game } }; // , end_menu::{ EndMenu }
// use heuristic;

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

	let mut events = Events::new(EventSettings::new());
	// .max_fps(100)
	// .lazy(true)
	
	let mut game = Game::new(opengl);
	let mut settings = Settings::new(opengl);
	// let mut end = EndMenu::new(opengl);
	let mut cursor = Cursor::new();

	let assets = Search::ParentsThenKids(3, 3).for_folder("resources").unwrap();
	let ref font = assets.join("DejaVuSerif.ttf");
	let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();
	let mut list_of_maps: Vec<Map> = vec![];

	while let Some(e) = events.next(&mut window)
	{
		if let Some(button) = e.press_args()
		{
			if cursor.controller == Controls::Mouse || cursor.controller == Controls::GameControls
			{
				if button == Button::Mouse(MouseButton::Left)
				{
					cursor.place_piece = false;
					cursor.press = true;
				}
			}
		}
		
		if let Some(button) = e.release_args()
		{
			// println!("button = {:?}", button);

			if cursor.controller == Controls::Mouse || cursor.controller == Controls::GameControls
			{
				if button == Button::Mouse(MouseButton::Left)
				{
					cursor.place_piece = true;
					cursor.press = false;
				}
			}
			if cursor.controller == Controls::GameControls
			{
				if button == Button::Keyboard(Key::P)
				{
					cursor.undo = true;
				}
				// else if button == Button::Keyboard(Key::Equals)
				// {
				// }
			}
			if cursor.controller == Controls::KeyBoard
			{
				if button == Button::Keyboard(Key::Up)
				{
					cursor.up = true;
				}
				else if button == Button::Keyboard(Key::Down)
				{
					cursor.down = true;
				}

				if button == Button::Keyboard(Key::Return)
				{
					cursor.press = true;
				}
			}
		}

		if let Some(pos) = e.mouse_cursor_args()
		{
			cursor.cursor_pos = pos;
		}

		if let Some(r) = e.render_args()
		{
			match cursor.selected_scene
			{
				Scene::Game => game.render(&r, &mut glyph_cache, &mut cursor, &mut list_of_maps),
				Scene::Settings => settings.render(&r, &mut glyph_cache, &mut cursor, &mut game.map),
				// Scene::End => end.render(&r, &mut glyph_cache, &mut cursor, &mut game.map, &mut list_of_maps),
			};
		}
	}
}
