use gomoku::player::{Player};
use gomoku::direction::{Direction};

const  SIZEMAP: usize = 19;

macro_rules! mapinit
{
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

#[derive(Debug, PartialEq)]
pub enum Slot
{
    PlayerOne,
    PlayerTwo,
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
            value: mapinit![SIZEMAP],
        }
    }
}

impl Map
{
    pub fn is_available(&self, (x, y):(i32, i32), player: Player) -> bool
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return false;
        }
        match self.value[y as usize][x as usize]
        {
            Slot::Empty => !self.is_double_three_move((x, y), player),
            _           => false
        }
    }

    pub fn move_authorize(&self, x: i32, y: i32, dir: Direction, player: Player) -> bool
    {
        self.is_available(dir.new_coordonate(x, y), player)
    }

    fn is_double_three_move(&self, (x, y):(i32, i32), player: Player) -> bool
    {
        false
    }
}
