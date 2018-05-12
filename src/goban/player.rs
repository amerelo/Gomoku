#[derive(Debug, Clone, PartialEq)]
pub enum PlayerKind
{
    Human,
    AI(i128),
}

#[derive(Debug, Clone)]
pub enum Player
{
    One,
    Two,
}

impl PartialEq for Player
{
    fn eq(&self, other: &Player) -> bool
    {
        match (self, other)
        {
            (&Player::One, &Player::One ) => true,
            (&Player::Two, &Player::Two ) => true,
            _                             => false,
        }
    }
}

impl Player
{
    pub fn to_string(&self, turn: usize) -> String
    {
        match self
        {
            &Player::One => format!["Turn {} for {}", turn, "Player One :\n"],
            _            => format!["Turn {} for {}", turn, "Player Two :\n"],
        }
    }
}

impl PlayerKind
{
    pub fn depth(&self) -> i128
    {
        match self
        {
            PlayerKind::AI(d) => *d,
            _                 => 0,
        }
    }

}