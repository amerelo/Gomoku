pub enum Direction
{
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
}

impl Direction
{
    pub fn new_coordonate(&self, (x, y): (i32, i32)) -> (i32, i32)
    {
        match self
        {
            &Direction::Up        => (x, y - 1),
            &Direction::UpLeft    => (x - 1, y - 1),
            &Direction::UpRight   => (x + 1, y - 1),
            &Direction::Down      => (x, y + 1),
            &Direction::DownLeft  => (x - 1, y + 1),
            &Direction::DownRight => (x + 1, y + 1),
            &Direction::Left      => (x - 1, y),
            &Direction::Right     => (x + 1, y),
        }
    }
}
