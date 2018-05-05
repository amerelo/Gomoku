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
macro_rules! insert_without_double
{
    ($n:expr, $vec:expr) =>
    {
        match $vec.binary_search(&$n)
        {
            Ok(pos) => {},
            Err(pos) => $vec.insert(pos, $n),
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
macro_rules! slot_capture
{
    ($slot:expr, $mov:expr; $array:expr; $value:expr ) =>
    {
        (($slot & ($array[$value].0 << ($mov - $array[$value].2 ))) >> ($mov - $array[$value].2 )) == $array[$value].1
    }
}

#[macro_export]
macro_rules! slot_value
{
    ( $slot:expr, $mov:expr; $array:expr; [$( $x:expr ),*] ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                if (($slot & ($array[$x].0 << ($mov - $array[$x].2 ))) >> ($mov - $array[$x].2 )) == $array[$x].1
                {
                    temp_vec.push($array[$x].3);
                }
            )*
            temp_vec.iter().fold(0, |sum, x| sum + x)
        }
    };
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
