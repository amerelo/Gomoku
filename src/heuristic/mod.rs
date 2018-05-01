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
