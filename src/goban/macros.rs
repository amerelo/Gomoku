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
    ($n:expr , $one:expr, $two:expr) =>
    {
        match $n
        {
            Player::One => $one,
            _           => $two
        };
    }
}

#[macro_export]
macro_rules! find_slot_enemy
{
    ($n:expr, $one:expr, $two:expr) =>
    {
        match $n
        {
            Player::One => $two,
            _           => $one
        };
    }
}

#[macro_export]
macro_rules! find_player
{
    ($n:expr) =>
    {
        match $n
        {
            HintSlot::CapturePlayerOne => Player::One,
            _                          => Player::Two
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
            Player::One => (&Slot::PlayerOne, &Slot::PlayerTwo),
            _           => (&Slot::PlayerTwo, &Slot::PlayerOne)
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
            if *slot.1 != *$player || $map.is_capturable(slot.0, ($player, $enemy))
            {
                break
            }
            count += 1;
        }
        count
    }}
}
