use goban::direction::{Direction};
use goban::map::{Map, slot::Slot};

const  WIN: i32 = 100;
const  IS_CAPTURED: i32 = -5;
const  U_CAPTURE: i32 = 5;
const  U_ALIGN: i32 = 2;
const  U_CUT: i32 = 2;


pub fn current_value(map: &mut Map, (x, y):(i32, i32), (slot_player, slot_enemy): (&Slot, &Slot)) -> i32
{
    let mut value: i32 = 0;

    if map.is_winning_move((x, y))
    {
        return WIN;
    }
    if map.is_capturable((x, y), (slot_player, slot_enemy))
    {
        value += IS_CAPTURED;
    }
    // add check score >= 10 with capture
    let captured = map.number_captured((x, y), (slot_player, slot_enemy), false);
    value += captured as i32 * U_CAPTURE;

    let aligned = map.number_aligned((x, y), (slot_player, slot_enemy));
    value += aligned as i32 * U_ALIGN;

    let cut = map.number_cut((x, y), (slot_player, slot_enemy));
    value += cut as i32 * U_CUT;

    value
}

fn map_value_vertical(map: &mut Map, (slot_player, slot_enemy): (&Slot, &Slot)) -> i32
{
    let mut value: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32;
    let mut empty_before: bool;

    while x < 19
    {
        y = 0;
        empty_before = false;
        while y < 19
        {
            match &map.value[y as usize][x as usize]
            {
                slot if slot == &Slot::Empty => { y += 1; empty_before = true; },
                slot if slot == slot_player  => { let tuple = map.align_value(&Direction::Down, (x, y), (slot, slot_enemy), empty_before); y += tuple.1; value += tuple.0; }
                slot if slot == slot_enemy   => { let tuple = map.align_value(&Direction::Down, (x, y), (slot, slot_player), empty_before); y += tuple.1; value -= tuple.0; }
                _                            => { y += 1 }
            }
        }
        x += 1;
    }
    value
}

fn map_value_horizontal(map: &mut Map, (slot_player, slot_enemy): (&Slot, &Slot)) -> i32
{
    let mut value: i32 = 0;
    let mut x: i32;
    let mut y: i32 = 0;
    let mut empty_before: bool;

    while y < 19
    {
        x = 0;
        empty_before = false;
        while x < 19
        {
            match &map.value[y as usize][x as usize]
            {
                slot if slot == &Slot::Empty => { x += 1; empty_before = true; },
                slot if slot == slot_player  => { let tuple = map.align_value(&Direction::Right, (x, y), (slot, slot_enemy), empty_before); x += tuple.1; value += tuple.0; }
                slot if slot == slot_enemy   => { let tuple = map.align_value(&Direction::Right, (x, y), (slot, slot_player), empty_before); x += tuple.1; value -= tuple.0; }
                _                            => { x += 1 }
            }
        }
        y += 1;
    }
    value
}

fn map_value_diagonal(map: &mut Map, (slot_player, slot_enemy): (&Slot, &Slot)) -> i32
{
    let mut value: i32 = 0;
    let mut x: i32;
    let mut y: i32 = 0;
    let mut c_bis: i32;
    let mut empty_before: bool;

    while y < 19
    {
        x = 0;
        c_bis = y;
        empty_before = false;
        while x < 19 && c_bis < 19
        {
            match &map.value[c_bis as usize][x as usize]
            {
                slot if slot == &Slot::Empty => { x += 1; c_bis += 1; empty_before = true; },
                slot if slot == slot_player  => { let tuple = map.align_value(&Direction::DownRight, (x, c_bis), (slot, slot_enemy), empty_before); x += tuple.1; c_bis += tuple.1; value += tuple.0; }
                slot if slot == slot_enemy   => { let tuple = map.align_value(&Direction::DownRight, (x, c_bis), (slot, slot_player), empty_before); x += tuple.1; c_bis += tuple.1; value -= tuple.0; }
                _                            => { x += 1; c_bis += 1 }
            }
        }
        y += 1;
    }

    x = 0;
    while x < 19
    {
        y = 0;
        c_bis = x;
        empty_before = false;
        while y < 19 && c_bis < 19
        {
            match &map.value[y as usize][c_bis as usize]
            {
                slot if slot == &Slot::Empty => { y += 1; c_bis += 1; empty_before = true; },
                slot if slot == slot_player  => { let tuple = map.align_value(&Direction::DownRight, (c_bis, y), (slot, slot_enemy), empty_before); y += tuple.1; c_bis += tuple.1; value += tuple.0; }
                slot if slot == slot_enemy   => { let tuple = map.align_value(&Direction::DownRight, (c_bis, y), (slot, slot_player), empty_before); y += tuple.1; c_bis += tuple.1; value -= tuple.0; }
                _                            => { y += 1; c_bis += 1 }
            }
        }
        x += 1;
    }

    y = 18;
    while y > -1
    {
        x = 18;
        c_bis = y;
        empty_before = false;
        while x > -1 && c_bis > -1
        {
            match &map.value[c_bis as usize][x as usize]
            {
                slot if slot == &Slot::Empty => { x -= 1; c_bis -= 1; empty_before = true; },
                slot if slot == slot_player  => { let tuple = map.align_value(&Direction::DownLeft, (x, c_bis), (slot, slot_enemy), empty_before); x -= tuple.1; c_bis -= tuple.1; value += tuple.0; }
                slot if slot == slot_enemy   => { let tuple = map.align_value(&Direction::DownLeft, (x, c_bis), (slot, slot_player), empty_before); x -= tuple.1; c_bis -= tuple.1; value -= tuple.0; }
                _                            => { x -= 1; c_bis -= 1 }
            }
        }
        y -= 1;
    }

    x = 18;
    while x > -1
    {
        y = 18;
        c_bis = x;
        empty_before = false;
        while y > -1 && c_bis > -1
        {
            match &map.value[y as usize][c_bis as usize]
            {
                slot if slot == &Slot::Empty => { y -= 1; c_bis -= 1; empty_before = true; },
                slot if slot == slot_player  => { let tuple = map.align_value(&Direction::DownLeft, (c_bis, y), (slot, slot_enemy), empty_before); y -= tuple.1; c_bis -= tuple.1; value += tuple.0; }
                slot if slot == slot_enemy   => { let tuple = map.align_value(&Direction::DownLeft, (c_bis, y), (slot, slot_player), empty_before); y -= tuple.1; c_bis -= tuple.1; value -= tuple.0; }
                _                            => { y -= 1; c_bis -= 1 }
            }
        }
        x -= 1;
    }

    value
}

pub fn map_value(map: &mut Map, (slot_player, slot_enemy): (&Slot, &Slot)) -> i32
{
    let mut value: i32 = 0;

    value += map_value_vertical(map, (slot_player, slot_enemy));
    println!("{}", value);
    value += map_value_horizontal(map, (slot_player, slot_enemy));
    println!("{}", value);
    value += map_value_diagonal(map, (slot_player, slot_enemy));
    println!("{}", value);
    value
}
