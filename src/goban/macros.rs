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

macro_rules! map_diagonale_init
{
    ($n:expr, $val:expr) =>
    {{
        let mut map = Vec::new();
        for _y in 0..$n
        {
            map.push($val);
        }
        for y in 0..19
        {
            for x in 0..19
            {
                let conv:(i128, i128) = match x >= y as i128
                {
                    true => (18 + (x - y as i128) as i128, (x + y as i128)as i128),
                    _    => (18 - (y as i128 - x) as i128, (x + y as i128)as i128)
                };
                if x == 0
                {
                    if y > 0
                    {
                        map[conv.1 as usize] ^= (0o33 as i128) << (3 * conv.0);
                    }
                    else
                    {
                        map[conv.1 as usize] ^= (0o3 as i128) << (3 * conv.0);
                    }

                }
                else if y == 0
                {
                    if x == 18
                    {
                        map[conv.1 as usize] ^= (0o3 as i128) << (3 * conv.0);
                    }
                    else
                    {
                        map[conv.1 as usize] ^= (0o3 as i128) << (3 * conv.0);
                    }

                }
                else if y == 18
                {
                    if x < 18
                    {
                        map[conv.1 as usize] ^= (0o33 as i128) << (3 * conv.0);
                    }
                    else
                    {
                        map[conv.1 as usize] ^= (0o3 as i128) << (3 * conv.0);
                    }

                }
                else if x == 18
                {
                    map[conv.1 as usize] ^= (0o3 as i128) << (3 * conv.0);
                }
                else
                {
                    map[conv.1 as usize] ^= (0o33 as i128) << (3 * conv.0);
                }
            }
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
    ($n:expr, $vec:expr, $map:expr, $player:expr) =>
    {
        if $map.is_available(($n.1, $n.0), $player) == 0
        {
            match $vec.binary_search(&$n)
            {
                Ok(_) => {},
                Err(pos) => $vec.insert(pos, $n),
            };
        }
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
macro_rules! find_kind_player
{
    ($n:expr, $kinds:expr) =>
    {
        match $n
        {
            Player::One => &$kinds.0,
            _           => &$kinds.1
        };
    }
}

#[macro_export]
macro_rules! find_kind_enemy
{
    ($n:expr) =>
    {
        match $n
        {
            Player::One => &Player::Two,
            _           => &Player::One
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
            Player::One => $scores.0 as i32,
            _           => $scores.1 as i32
        };
    }
}

#[macro_export]
macro_rules! find_enemy_score
{
    ($n:expr, $scores:expr) =>
    {
        match $n
        {
            Player::Two => $scores.0 as i32,
            _           => $scores.1 as i32
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

macro_rules! slot_capturable
{
    ($slot:expr, $mov:expr; $array:expr; [$($value:expr),*] ) =>
    {
        $((($slot & ($array[$value].0 << ($mov - $array[$value].2 ))) >> ($mov - $array[$value].2 )) == $array[$value].1)||*
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
