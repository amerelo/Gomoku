use gomoku::player::{Player};

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
    pub fn is_available(&self, x: usize, y: usize) -> bool
    {
        if x > 18 || y > 18
        {
            return false;
        }
        match self.value[y][x]
        {
            Slot::Empty => true,
            _           => false
        }
    }
}

#[cfg(test)]
mod tests
{
	use super::*;
    use gomoku::player::{Player, PlayerKind};

    fn init() -> Map
    {
        let mut test = Map {..Default::default() };

        test.value[0][3] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[3][0] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[2][3] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[12][3] = Slot::Used(Player::Two(PlayerKind::Human));
        test.value[13][4] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[2][5] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[11][3] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[11][3] = Slot::Used(Player::One(PlayerKind::AI));
        test
    }
    
	#[test]
	fn slot_is_available_0()
    {
		assert_eq!(init().is_available(0, 2), true);
	}

	#[test]
	fn slot_is_unavailable_0()
    {
        let test = init();

		assert_eq!(test.is_available(0, 3), false);
		assert_eq!(test.is_available(19, 4), false); // overflow 
	}
}
