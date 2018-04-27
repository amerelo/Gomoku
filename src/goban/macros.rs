macro_rules! mapinit
{
    ($n:expr) => {{
        let mut map = Vec::new();
        for _y in 0..$n {
            let mut vec = Vec::new();
            for _x in 0..$n {
                vec.push(Slot::Empty);
            }
            map.push(vec)
        }
        map
    }}
}

macro_rules! find_slot_player
{
    ($n:expr) => {
        match $n {
            Player::One => Slot::PlayerOne,
            _           => Slot::PlayerTwo
        };
    }
}

macro_rules! slot_cmp
{
    (
        $slot:expr; [ $( $value:expr ),* ]
    ) => {
        $($slot == $value)||*
    }
}
