use goban::map::{ Map, constant::{SIZEMAP, RSIZEMAP, MOVE_P1, MOVE_P2, DMOVE_P1, DMOVE_P2} };
use goban::player::{Player};

pub fn value_slot(map: &Map, (y, x, _):(i128, i128, i128)) -> i128
{
    let mut count:i128 = 0;


    if x <= 0 || y <= 0 || x >= RSIZEMAP || y >= RSIZEMAP
    {
        return 0;
    }

    count += match x
    {
        17 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 3, 6]],
        16 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 1, 3, 4]],
        15 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 7]],
        _  => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 5, 7, 8]]
    };

    count += match y
    {
        1 => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 3, 6]],
        2 => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 1, 3, 4]],
        3 => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 7]],
        _ => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 5, 7, 8]]
    };

    // count += match x
    // {
    //     17 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 3, 6]],
    //     16 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 1, 3, 4]],
    //     15 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 7]],
    //     _  => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 5, 7, 8]]
    // };

    // count += match y
    // {
    //     1 => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 3, 6]],
    //     2 => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 1, 3, 4]],
    //     3 => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 7]],
    //     _ => slot_value![map.value_rotate[x as usize], y * 3 ; MOVE_P1; [0, 1, 2, 3, 4, 5, 7, 8]]
    // };

    // let conv:(i128, i128) = match x >= y
    // {
    //     true => (RSIZEMAP + (x - y) as i128, (x + y)as i128),
    //     _    => (RSIZEMAP - (y - x) as i128, (x + y)as i128)
    // };

    // if conv.1 < 3 || conv.1 > 33 || conv.0 < 3 || conv.0 > 33
    // {
    // println!("x {} y {} count {}", x, y, count);
    //     return count as i128;
    // }

    // count += match conv.0
    // {
    //     3 ... 5 => slot_cmp![map.value_diagonale[conv.1 as usize], conv.0 * 3 ; DMOVE_P2; [0, 3, 6]] as usize,
    //     6 | 7 => slot_cmp![map.value_diagonale[conv.1 as usize], conv.0 * 3 ; DMOVE_P2; [0, 1, 2, 3, 4, 6, 7]] as usize,
    //     _ => slot_cmp![map.value_diagonale[conv.1 as usize], conv.0 * 3 ; DMOVE_P2; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
    // };

    // count += match (RSIZEMAP as i128) * 2 - conv.1
    // {
    //     3 ... 5 => slot_cmp![map.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; DMOVE_P2; [0, 3, 6]] as usize,
    //     6 | 7 => slot_cmp![map.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; DMOVE_P2; [0, 1, 2, 3, 4, 6, 7]] as usize,
    //     _ => slot_cmp![map.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; DMOVE_P2; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
    // };
    // println!("x {} y {} count {}", x, y, count);
    count as i128
}