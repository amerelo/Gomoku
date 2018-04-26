pub enum Direction
{
    Up,
    Down,
    Left,
    Right,
}

impl Direction
{
    pub fn new_coordonate(&self, x: i32, y: i32) -> (i32, i32)
    {
        match self
        {
            &Direction::Up => (x, y - 1),
            &Direction::Down => (x, y + 1),
            &Direction::Left => (x - 1, y),
            &Direction::Right => (x + 1, y),
        }
    }
}