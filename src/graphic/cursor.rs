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
	pub cursor_pos: [f64; 2],
	pub cursor_in_board: [usize; 2],
	pub selected_scene: Scene,
	pub controller: Controls,
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
			hint: true,
			cursor_pos: [0.0, 0.0],
			cursor_in_board: [0, 0],
			selected_scene: Scene::Settings,
			controller: Controls::KeyBoard,
		}
	}
}