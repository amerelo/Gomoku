use goban::map::{Map};
use std::slice::Iter;

#[derive(Debug)]
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
    pub fn new_coordonate(&self, (x, y): (i128, i128)) -> (i128, i128)
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

    pub fn next_four(&self, (x, y): (i128, i128), map: & Map) -> (i128, i128, i128, i128)
    {
        let one = self.new_coordonate((x, y));
        let two = self.new_coordonate(one);
        let three = self.new_coordonate(two);
        let four = self.new_coordonate(three);

        (map.find_value(one), map.find_value(two), map.find_value(three), map.find_value(four))
    }

    pub fn next_three(&self, (x, y): (i128, i128), map: & Map) -> (i128, i128, i128)
    {
        let one = self.new_coordonate((x, y));
        let two = self.new_coordonate(one);
        let three = self.new_coordonate(two);

        (map.find_value(one), map.find_value(two), map.find_value(three))
    }

    pub fn next_two(&self, (x, y): (i128, i128), map: & Map) -> (i128, i128)
    {
        let one = self.new_coordonate((x, y));
        let two = self.new_coordonate(one);

        (map.find_value(one), map.find_value(two))
    }

    pub fn capture(&self, (x, y): (i128, i128), map: &mut Map) -> ()
    {
        let one = self.new_coordonate((x, y));
        let two = self.new_coordonate(one);

        map.value[one.1 as usize] &= 0o0 << (one.0 * 3);
        map.value[two.1 as usize] &= 0o0 << (two.0 * 3);
    }

    pub fn iterator() -> Iter<'static, Direction>
    {
        static DIRECTIONS: [Direction;  8] = [Direction::Up, Direction::UpLeft, Direction::UpRight, Direction::Down, Direction::DownLeft, Direction::DownRight, Direction::Left, Direction::Right];
        DIRECTIONS.into_iter()
    }

    pub fn axes_iterator() -> Iter<'static, (Direction, Direction)>
    {
        static DIRECTIONS: [(Direction, Direction);  4] = [(Direction::Up, Direction::Down), (Direction::UpLeft, Direction::DownRight), (Direction::UpRight, Direction::DownLeft), (Direction::Left, Direction::Right)];
        DIRECTIONS.into_iter()
    }
}
