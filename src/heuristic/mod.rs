use std::i128::{MIN, MAX};
use std::usize;
use goban::map::{ Map, constant::{*} };
use goban::player::{Player};
use goban::finish::{Finish};

pub fn value_slot(map: &Map, (y, x, _):(i128, i128, i128), player: &Player) -> i128
{
    let mut count:i128 = 0;
    let masks_move = find_tm_player![player, (MOVE_P1, DMOVE_P1), (MOVE_P2, DMOVE_P2)];

    if x < 0 || y < 0 || x > RSIZEMAP || y > RSIZEMAP
    {
        return 0;
    }

    let conv:(i128, i128) = match x >= y
    {
        true => (RSIZEMAP + (x - y) as i128, (x + y) as i128),
        _    => (RSIZEMAP - (y - x) as i128, (x + y) as i128)
    };

    count += value_slot_x(map, (x, y), masks_move.0, (RSIZEMAP - x) * 3);
    count += value_slot_y(map, (x, y), masks_move.0, y * 3);
    count += value_slot_diagonale_x(map, conv, masks_move.1, conv.0 * 3);
    count += value_slot_diagonale_y(map, conv, masks_move.1, ((RSIZEMAP as i128) * 2 - conv.1) * 3);

    count as i128
}

pub fn value_map(map: &Map, slot: &Player) -> i128
{
	let mut count:i128 = 0;

	match (&map.is_finish, slot)
    {
        (&Finish::None, _) => {},
        (&Finish::CapturePlayerOne, &Player::One) => { return MAX / 2 },
        (&Finish::CapturePlayerOne, &Player::Two) => { return MIN / 2 },
        (&Finish::CapturePlayerTwo, &Player::One) => { return MIN / 2 },
        (&Finish::CapturePlayerTwo, &Player::Two) => { return MAX / 2 },
        (&Finish::AlignPlayerOne, &Player::Two)   => { return MIN / 2 },
        (&Finish::AlignPlayerOne, &Player::One)   => { return MAX / 2 },
        (&Finish::AlignPlayerTwo, &Player::Two)   => { return MAX / 2 },
        (&Finish::AlignPlayerTwo, &Player::One)   => { return MIN / 2 },
    }
	count += find_score![slot, map.players_score] as i128 * CAPTURE * 2;
	count -= find_enemy_score![slot, map.players_score] as i128 * CAPTURE * 2;
	count += sum_value_slot(map, slot);

    count
}

fn value_slot_x(map: &Map, (x, y): (i128, i128), mask_move: [(i128, i128, i128, i128); 35], index: i128) -> i128
{
	let mut count:i128 = 0;

    count += match x // capture
    {
        18 => slot_value![map.value[y as usize], index ; mask_move; [9]],
        17 => slot_value![map.value[y as usize], index ; mask_move; [9]],
        16 => slot_value![map.value[y as usize], index ; mask_move; [9]],
        15 => slot_value![map.value[y as usize], index ; mask_move; [9]],
        _  => slot_value![map.value[y as usize], index ; mask_move; [9, 10]]
    };

    count += match x // five align
    {
        18 => slot_value![map.value[y as usize], index ; mask_move; [25, 30]],
        17 => slot_value![map.value[y as usize], index ; mask_move; [25, 26, 30, 31]],
        16 => slot_value![map.value[y as usize], index ; mask_move; [25, 26, 27, 30, 31, 32]],
        15 => slot_value![map.value[y as usize], index ; mask_move; [25, 26, 27, 28, 30, 31, 32, 33]],
        _  => slot_value![map.value[y as usize], index ; mask_move; [25, 26, 27, 28, 29, 30, 31, 32, 33, 34]]
    };

    if count > 0
    {
        return count;
    }

    count += match x // four align
    {
        18 => slot_value![map.value[y as usize], index ; mask_move; [11, 21]],
        17 => slot_value![map.value[y as usize], index ; mask_move; [11, 17, 21, 22]],
        16 => slot_value![map.value[y as usize], index ; mask_move; [11, 17, 18, 21, 22, 23]],
        15 => slot_value![map.value[y as usize], index ; mask_move; [11, 17, 18, 19, 21, 22, 23, 24]],
        _  => slot_value![map.value[y as usize], index ; mask_move; [11, 12, 17, 18, 19, 20, 21, 22, 23, 24]]
    };

    if count > 0
    {
        return count;
    }

    count += match x // three align
    {
        18 => slot_value![map.value[y as usize], index ; mask_move; [0, 3, 6, 13]],
        17 => slot_value![map.value[y as usize], index ; mask_move; [0, 3, 6, 13, 15]],
        16 => slot_value![map.value[y as usize], index ; mask_move; [0, 1, 3, 4, 13, 15, 16, 18, 23]],
        15 => slot_value![map.value[y as usize], index ; mask_move; [0, 1, 2, 3, 4, 7, 13, 14, 15, 16, 18, 19, 23]],
        _  => slot_value![map.value[y as usize], index ; mask_move; [0, 1, 2, 3, 4, 5, 7, 8, 12, 13, 14, 15, 16, 18, 19, 20, 23]]
    };

    count
}

fn value_slot_y(map: &Map, (x, y): (i128, i128), mask_move: [(i128, i128, i128, i128); 35], index: i128) -> i128
{
	let mut count:i128 = 0;

    count += match y // capture
    {
        0 => slot_value![map.value_rotate[x as usize], index ; mask_move; [9]],
        1 => slot_value![map.value_rotate[x as usize], index ; mask_move; [9]],
        2 => slot_value![map.value_rotate[x as usize], index ; mask_move; [9]],
        3 => slot_value![map.value_rotate[x as usize], index ; mask_move; [9]],
        _ => slot_value![map.value_rotate[x as usize], index ; mask_move; [9, 10]]
    };

    count += match y // five align
    {
        0 => slot_value![map.value_rotate[x as usize], index ; mask_move; [25, 30]],
        1 => slot_value![map.value_rotate[x as usize], index ; mask_move; [25, 26, 30, 31]],
        2 => slot_value![map.value_rotate[x as usize], index ; mask_move; [25, 26, 27, 30, 31, 32]],
        3 => slot_value![map.value_rotate[x as usize], index ; mask_move; [25, 26, 27, 28, 30, 31, 32, 33]],
        _ => slot_value![map.value_rotate[x as usize], index ; mask_move; [25, 26, 27, 28, 29, 30, 31, 32, 33, 34]]
    };

    if count > 0
    {
        return count;
    }

    count += match y // four align
    {
        0 => slot_value![map.value_rotate[x as usize], index ; mask_move; [11, 21]],
        1 => slot_value![map.value_rotate[x as usize], index ; mask_move; [11, 17, 21, 22]],
        2 => slot_value![map.value_rotate[x as usize], index ; mask_move; [11, 17, 18, 21, 22, 23]],
        3 => slot_value![map.value_rotate[x as usize], index ; mask_move; [11, 17, 18, 19, 21, 22, 23, 24]],
        _ => slot_value![map.value_rotate[x as usize], index ; mask_move; [11, 12, 17, 18, 19, 20, 21, 22, 23, 24]]
    };

    if count > 0
    {
        return count;
    }

    count += match y // three align
    {
        0 => slot_value![map.value_rotate[x as usize], index ; mask_move; [0, 3, 6, 13]],
        1 => slot_value![map.value_rotate[x as usize], index ; mask_move; [0, 3, 6, 13, 15]],
        2 => slot_value![map.value_rotate[x as usize], index ; mask_move; [0, 1, 3, 4, 13, 15, 16, 18, 23]],
        3 => slot_value![map.value_rotate[x as usize], index ; mask_move; [0, 1, 2, 3, 4, 7, 13, 14, 15, 16, 18, 19, 23]],
        _ => slot_value![map.value_rotate[x as usize], index ; mask_move; [0, 1, 2, 3, 4, 5, 7, 8, 12, 13, 14, 15, 16, 18, 19, 20, 23]]
    };

    count
}

fn value_slot_diagonale_x(map: &Map, (x, y): (i128, i128), mask_move: [(i128, i128, i128, i128); 35], index: i128) -> i128
{
	let mut count:i128 = 0;

    count += match x // capture
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale[y as usize], index ; mask_move; [9]],
        _       => slot_value![map.value_diagonale[y as usize], index ; mask_move; [9, 10]],
    };

    count += match x // five align
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale[y as usize], index ; mask_move; [25, 26, 30, 31]],
        6 | 7 => slot_value![map.value_diagonale[y as usize], index ; mask_move; [25, 26, 27, 28, 30, 31, 32, 33]],
        _ => slot_value![map.value_diagonale[y as usize], index ; mask_move; [25, 26, 27, 28, 29, 30, 31, 32, 33, 34]],
    };

    if count > 0
    {
        return count;
    }

    count += match x // four align
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale[y as usize], index ; mask_move; [11, 17, 21, 22]],
        6 | 7 => slot_value![map.value_diagonale[y as usize], index ; mask_move; [11, 17, 18, 19, 21, 22, 23, 24]],
        _ => slot_value![map.value_diagonale[y as usize], index ; mask_move; [11, 12, 17, 18, 19, 20, 21, 22, 23, 24]]
    };

    if count > 0
    {
        return count;
    }

    count += match x // three align
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale[y as usize], index ; mask_move; [0, 1, 3, 4, 13, 15]],
        6 | 7 => slot_value![map.value_diagonale[y as usize], index ; mask_move; [0, 1, 3, 4, 13, 15, 16, 18, 19, 23, 24]],
        _ => slot_value![map.value_diagonale[y as usize], index ; mask_move; [0, 1, 2, 3, 4, 5, 7, 8, 12, 13, 14, 15, 16, 18, 19, 20, 23]],
    };

    count
}

fn value_slot_diagonale_y(map: &Map, (x, _): (i128, i128), mask_move: [(i128, i128, i128, i128); 35], index: i128) -> i128
{
	let mut count:i128 = 0;
    let m = index / 3;

    count += match m // capture
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [9]],
        _       => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [9, 10]],
    };

    count += match m // five align
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [25, 26, 30, 31]],
        6 | 7 => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [25, 26, 27, 28, 30, 31, 32, 33]],
        _ => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [25, 26, 27, 28, 29, 30, 31, 32, 33, 34]],
    };

    if count > 0
    {
        return count;
    }

    count += match m // four align
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [11, 17, 21, 22]],
        6 | 7 => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [11, 17, 18, 19, 21, 22, 23, 24]],
        _ => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [11, 12, 17, 18, 19, 20, 21, 22, 23, 24]]
    };

    if count > 0
    {
        return count;
    }

    count += match m // three align
    {
        0 ... 2 => 0,
        3 ... 5 => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [0, 1, 3, 4, 13, 15]],
        6 | 7 => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [0, 1, 3, 4, 13, 15, 16, 18, 19, 23, 24]],
        _ => slot_value![map.value_diagonale_rotate[x as usize], index ; mask_move; [0, 1, 2, 3, 4, 5, 7, 8, 12, 13, 14, 15, 16, 18, 19, 20, 23]],
    };

    count
}

fn sum_value_slot(map: &Map, player: &Player) -> i128
{
	let area = map.area_of_interest(usize::MAX, player);
	let mut count:i128 = 0;

    for (_, _, value) in area
    {
        count += value;
    }
    count
}
