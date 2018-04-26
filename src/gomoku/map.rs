use gomoku::player::{Player, PlayerKind};
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
    pub players: (Player, Player),
    pub current_player: Player,
}

impl Default for Map
{
    fn default() -> Map
    {
        Map
        {
            value: mapinit![SIZEMAP],
            players: (Player::One(PlayerKind::Human), Player::Two(PlayerKind::Human)),
            current_player: Player::One(PlayerKind::Human),
        }
    }
}

impl Map
{
    pub fn is_available(&self, (x, y):(i32, i32), player: Player) -> Slot
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return Slot::Forbidden;
        }
        match self.value[y as usize][x as usize]
        {
            Slot::Empty     => self.is_double_three_move((x, y), player),
            Slot::PlayerOne => Slot::PlayerOne,
            _               => Slot::PlayerTwo
        }
    }

    pub fn change_player_turn(&mut self)
    {
        match self.current_player
        {
            Player::One(_) => self.current_player = Player::Two(PlayerKind::Human),
            _              => self.current_player = Player::One(PlayerKind::Human)
        }
    }

    pub fn move_authorize(&self, x: i32, y: i32, dir: Direction, player: Player) -> bool
    {
        self.is_available(dir.new_coordonate(x, y), player) == Slot::Empty
    }

    fn is_double_three_move(&self, (x, y):(i32, i32), player: Player) -> Slot
    {
        Slot::Empty
    }
}
