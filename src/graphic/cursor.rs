#[derive(PartialEq,)]
pub enum Scene
{
	Settings,
	Game,
}

#[derive(PartialEq,)]
pub enum Controls
{
	Mouse,
	KeyBoard,
	GameControls,
}

pub struct Cursor
{
	pub press: bool,
	pub up: bool,
	pub down: bool,
	pub place_piece: bool,
	pub undo: bool,
	pub hint: bool,
	pub prev: bool,
	pub ai1_level: i128,
	pub ai2_level: i128,
	pub cursor_pos: [f64; 2],
	pub cursor_in_board: [usize; 2],
	pub selected_scene: Scene,
	pub controller: Controls,
	pub last_move_x_y: (i128, i128),
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
			undo: false,
			hint: false,
			prev: false,
			ai1_level: 6,
			ai2_level: 6,
			cursor_pos: [0.0, 0.0],
			cursor_in_board: [0, 0],
			selected_scene: Scene::Settings,
			controller: Controls::KeyBoard,
			last_move_x_y: (-1, -1),
		}
	}
}