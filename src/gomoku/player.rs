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
