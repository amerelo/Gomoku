use goban::map::{Map, Slot};

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

    pub fn next_three<'a>(&self, (x, y): (i32, i32), map: &'a Map) -> (&'a Slot, &'a Slot, &'a Slot)
    {
        let one = self.new_coordonate((x, y));
        let two = self.new_coordonate(one);
        let three = self.new_coordonate(two);

        (map.find_value(one), map.find_value(two), map.find_value(three))
    }

    pub fn capture(&self, (x, y): (i32, i32), map: &mut Map) -> ()
    {
        let one = self.new_coordonate((x, y));
        let two = self.new_coordonate(one);

        map.value[one.1 as usize][one.0 as usize] = Slot::Empty;
        map.value[two.1 as usize][two.0 as usize] = Slot::Empty;
    }
}
