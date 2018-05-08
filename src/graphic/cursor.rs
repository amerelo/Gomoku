pub enum Scene
{
	Settings,
	Game,
}

pub struct Cursor
{
	pub press: bool,
	pub up: bool,
	pub down: bool,
	pub place_piece: bool,
	pub cursor_pos: [f64; 2],
	pub cursor_in_board: [usize; 2],
	pub selected_scene: Scene,
}

impl Cursor
{
	pub fn new() -> Self
	{
		Cursor
		{
			press: false,
			up: false,
			down: false,
			place_piece: false,
			cursor_pos: [0.0, 0.0],
			cursor_in_board: [0, 0],
			selected_scene: Scene::Settings,
		}
	}
}