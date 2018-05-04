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
macro_rules! slot_cmp
{
    ($slot:expr, $mov:expr; $array:expr; [$($value:expr),*] ) =>
    {
        $((($slot & ($array[$value].0 << ($mov - $array[$value].2 ))) >> ($mov - $array[$value].2 )) == $array[$value].1)||*
    }
}

#[macro_export]
macro_rules! find_tm_player
{
    ($n:expr, $p1:expr, $p2:expr) =>
    {
        match $n
        {
            Player::One => $p1,
            _           => $p2
        };
    }
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
