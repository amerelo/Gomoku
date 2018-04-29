#[macro_export]
macro_rules! mapinit
{
    ($n:expr, $val:expr) =>
    {{
        let mut map = Vec::new();
        for _y in 0..$n
        {
            let mut vec = Vec::new();
            for _x in 0..$n
            {
                vec.push($val);
            }
            map.push(vec)
        }
        map
    }}
}

#[macro_export]
macro_rules! find_slot_player
{
    ($n:expr) =>
    {
        match $n
        {
            Player::One => Slot::PlayerOne,
            _           => Slot::PlayerTwo
        };
    }
}

#[macro_export]
macro_rules! find_slot_enemy
{
    ($n:expr) =>
    {
        match $n
        {
            Player::One => Slot::PlayerTwo,
            _           => Slot::PlayerOne
        };
    }
}

#[macro_export]
macro_rules! slot_cmp_or
{
    ($slot:expr; [ $( $value:expr ),* ]) =>
    {
        $($slot == $value)||*
    }
}

#[macro_export]
macro_rules! slot_cmp
{
    ($player:expr; $value:expr) =>
    {{
        let (player, p) = match $player {
            &Slot::PlayerOne => (&Slot::PlayerOne, &Slot::PlayerOne),
            _               => (&Slot::PlayerTwo, &Slot::PlayerTwo),
        };
        let (a, b, c, d) = $value;

        if (a, b, c) == (player, p, &Slot::Empty)
            || (a, b, c, d) == (player, &Slot::Empty, p, &Slot::Empty)
            || (a, b, c, d) == (&Slot::Empty, player, p, &Slot::Empty)
        {
            2
        }
        else if (a, b, c) == (&Slot::Empty, player, &Slot::Empty)
            || (a, b) == (player, &Slot::Empty)
        {
            1
        }
        else
        {
            0
        }
    }}
}
