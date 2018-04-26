#[derive(Debug)]
pub enum PlayerKind
{
    Human,
    AI,
}

#[derive(Debug)]
pub enum Player
{
    One(PlayerKind),
    Two(PlayerKind),
}

impl PartialEq for Player
{
    fn eq(&self, other: &Player) -> bool
    {
        match (self, other)
        {
            (&Player::One(_) , &Player::One(_) ) => true,
            (&Player::Two(_) , &Player::Two(_) ) => true,
            _                                    => false,
        }
    }
}