use goban::map::{ Map, constant::{*} };

use goban::player::{Player};
use std::i128::{MIN, MAX};

pub fn value_slot(map: &Map, (y, x, _):(i128, i128, i128)) -> i128
{
    let mut count:i128 = 0;
    let masks_move = find_tm_player![map.current_player, (MOVE_P1, DMOVE_P1), (MOVE_P2, DMOVE_P2)];

    let conv:(i128, i128) = match x >= y
    {
        true => (RSIZEMAP + (x - y) as i128, (x + y)as i128),
        _    => (RSIZEMAP - (y - x) as i128, (x + y)as i128)
    };

    if x < 0 || y < 0 || x > RSIZEMAP || y > RSIZEMAP
    {
        return 0;
    }

// println!("match x {} y {}", x, y);
    count += match x
    {
        18 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; masks_move.0; [0, 3, 6, 9, 11, 13, 21, 25, 30]],
        17 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; masks_move.0; [0, 3, 6, 9, 11, 13, 15, 17, 21, 22, 25, 26, 30, 31]],
        16 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; masks_move.0; [0, 1, 3, 4, 9, 11, 13, 15, 16, 17, 18, 21, 22, 23, 25, 26, 27, 30, 31, 32]],
        15 => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; masks_move.0; [0, 1, 2, 3, 4, 7, 9, 11, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 33]],
        _  => slot_value![map.value[y as usize], (RSIZEMAP - x) * 3 ; masks_move.0; [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34]]
    };

 // println!("x {} match y {}", x, y);
    count += match y
    {
        0 => slot_value![map.value_rotate[x as usize], y * 3 ; masks_move.0; [0, 3, 6, 9, 11, 13, 21, 25, 30]],
        1 => slot_value![map.value_rotate[x as usize], y * 3 ; masks_move.0; [0, 3, 6, 9, 11, 13, 15, 17, 21, 22, 25, 26, 30, 31]],
        2 => slot_value![map.value_rotate[x as usize], y * 3 ; masks_move.0; [0, 1, 3, 4, 9, 11, 13, 15, 16, 17, 18, 21, 22, 23, 25, 26, 27, 30, 31, 32]],
        3 => slot_value![map.value_rotate[x as usize], y * 3 ; masks_move.0; [0, 1, 2, 3, 4, 7, 9, 11, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 30, 31, 32, 33]],
        _ => slot_value![map.value_rotate[x as usize], y * 3 ; masks_move.0; [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34]]
    };

// println!("x {} y {} match conv.0 {} conv.1 {}", x, y, conv.0, conv.1);
    count += match conv.0
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale[conv.1 as usize], conv.0 * 3 ; masks_move.1; [0, 1, 3, 4, 6, 9, 11, 13, 15, 17, 21, 22, 25, 26, 30, 31]],
        6 | 7 => slot_value![map.value_diagonale[conv.1 as usize], conv.0 * 3 ; masks_move.1; [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 30, 31,32, 33]],
        _ => slot_value![map.value_diagonale[conv.1 as usize], conv.0 * 3 ; masks_move.1; [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34]]
    };


//    println!("x {} y {} match conv.0 {} conv.1 {}", x, y, conv.0, conv.1);
   count += match (RSIZEMAP as i128) * 2 - conv.1
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; masks_move.1; [0, 1, 3, 4, 6, 9, 11, 13, 15, 17, 21, 22, 25, 26, 30, 31]],
        6 | 7 => slot_value![map.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; masks_move.1; [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 21, 22, 23, 24, 25, 26, 27, 28, 30, 31,32, 33]],
        _ => slot_value![map.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; masks_move.1; [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34]]
    };

    // println!("x {} y {} count {}", x, y, count);
    count as i128
}

pub fn value_map(map: &Map) -> i128
{
    if map.is_finish
    {
        match find_score![map.current_player, map.players_score] >= 10
        {
            true => { return MAX },
            _    => { return MIN }
        }
    }
    0
}
