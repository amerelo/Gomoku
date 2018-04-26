use gomoku::player::{Player};
use gomoku::direction::{Direction};

const  SIZEMAP: usize = 19;

macro_rules! vecinit {
    ($n:expr) => {{
        let mut map = Vec::new();
        for _y in 0..$n {
            let mut vec = Vec::new();
            for _x in 0..$n {
                vec.push(Slot::Empty);
            }
            map.push(vec)
        }
        map
    }}
}

#[derive(Debug)]
pub enum Slot
{
    Used(Player),
    Empty,
    Forbidden,
}

#[derive(Debug)]
pub struct Map
{
    pub value: Vec<Vec<Slot>>,
}

impl Default for Map
{
    fn default() -> Map
    {
        Map {
            value: vecinit![SIZEMAP],
        }
    }
}

impl Map {
    pub fn is_available(&self, (x, y):(i32, i32)) -> bool
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return false;
        }
        match self.value[y as usize][x as usize]
        {
            Slot::Empty => true,
            _           => false
        }
    }

    pub fn move_authorize(&self, x: i32, y: i32, dir: Direction) -> bool
    {
        self.is_available(dir.new_coordonate(x, y))
    }
}
