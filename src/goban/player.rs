#[derive(Debug, Clone, PartialEq)]
pub enum PlayerKind
{
    Human,
    AI,
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
