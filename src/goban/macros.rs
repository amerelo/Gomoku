#[macro_export]
macro_rules! mapinit
{
    ($n:expr, $val:expr) =>
    {{
        let mut map = Vec::new();
        for _y in 0..$n
        {
            map.push($val);
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
            Player::One => 1,
            _           => 2
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
            Player::One => 2,
            _           => 1
        };
    }
}

#[macro_export]
macro_rules! find_score
{
    ($n:expr, $scores:expr) =>
    {
        match $n
        {
            &Slot::PlayerOne => $scores.0 as i32,
            _                => $scores.1 as i32
        };
    }
}

#[macro_export]
macro_rules! find_slots_players
{
    ($n:expr) =>
    {
        match $n
        {
            Player::One => (1, 2),
            _           => (2, 1)
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

#[macro_export]
macro_rules! slots_winning
{
    ($player:expr; $enemy:expr; $map:expr; $value:expr) =>
    {{
        let mut count:usize = 0;
    
        for slot in $value.iter()
        {
            if slot.1 != $player || $map.is_capturable(slot.0, ($player, $enemy))
            {
                break
            }
            count += 1;
        }
        count
    }}
}
