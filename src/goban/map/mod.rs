pub mod slot;

use goban::player::{Player, PlayerKind};
use goban::direction::{Direction};
use std::i128;

const  SIZEMAP: usize = 19;
const  RSIZEMAP: i64 = 18;

#[derive(Debug, Clone)]
pub struct Map
{
    pub value: Vec<i64>,
    pub value_rotate: Vec<i64>,
    pub value_diagonale: Vec<i128>,
    pub value_diagonale_rotate: Vec<i128>,
    pub players_kind: (PlayerKind, PlayerKind), // easier way for handling players number and players kind
    pub players_score: (usize, usize),
    pub current_player: Player,
    pub turn: usize,
    pub is_finish: bool,
}

impl Default for Map
{
    fn default() -> Map
    {
        Map
        {
            value: mapinit![SIZEMAP, 0 as i64],
            value_rotate: mapinit![SIZEMAP, 0 as i64],
            value_diagonale: mapinit![SIZEMAP * 2, 0 as i128],
            value_diagonale_rotate: mapinit![SIZEMAP * 2, 0 as i128],
            players_kind: (PlayerKind::Human, PlayerKind::Human),
            players_score: (0, 0),
            current_player: Player::One,
            turn: 1,
            is_finish: false,
        }
    }
}

impl Map
{
    pub fn reset(&mut self) -> ()
    {
        self.value = mapinit![SIZEMAP, 0];
        self.value_rotate = mapinit![SIZEMAP, 0];
        self.value_diagonale = mapinit![SIZEMAP * 2, 0];
        self.value_diagonale_rotate = mapinit![SIZEMAP * 2, 0];
        self.players_score = (0, 0);
        self.current_player = Player::One;
        self.is_finish = false;
        self.turn = 1;
    }

    pub fn set_value(&mut self, (x, y):(i64, i64), value: i64) -> ()
    {
        self.value[y as usize] ^= value << (3 * (RSIZEMAP - x));
        self.value_rotate[x as usize] ^= value << 3 * y;
        let conv:(i128, i128) = match x >= y
        {
            true => (18 + (x - y) as i128, (x + y)as i128), 
            _    => (18 - (y - x) as i128, (x + y)as i128)
        };
        self.value_diagonale[conv.1 as usize] ^= (value as i128) << 3 * conv.0;
        self.value_diagonale_rotate[conv.0 as usize] ^= (value as i128) << 3 * ((RSIZEMAP as i128) * 2 - conv.1);
    }

    pub fn is_available(&self, (x, y):(i64, i64)) -> i64
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return -1;
        }
        match (self.value[y as usize] & 0o3 << (3 * (RSIZEMAP - x))) >> 3 * (RSIZEMAP - x)
        {
            0     => self.is_double_three_move((x, y)),
            val   => val
        }
    }

    fn is_double_three_move(&self, (x, y):(i64, i64)) -> i64
    {
        // match self.three_move_number((x, y), find_slot_player![self.current_player])
        // {
        //     1 => 0,
        //     0 => 0,
        //     _ => -1
        // }
        0
    }

    // fn three_move_number(&self, (x, y):(i64, i64), slot_player: Slot) -> usize
    // {
    //     let mut count:usize = 0;
    //     let slot_enemy = find_slot_enemy![self.current_player];

    //     for axe in Direction::axes_iterator()
    //     {
    //         if self.is_free_three((x, y), (&slot_player, &slot_enemy), axe)
    //         {
    //             count += 1;
    //             if count == 2
    //             {
    //                 return 2
    //             }
    //         }
    //     }
    //     count
    // }

    // fn is_free_three(&self, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot), &(ref dir_add, ref dir_sub): &(Direction, Direction)) -> bool
    // {
    //     let mut add = dir_add.new_coordonate((x, y));
    //     let mut sub = dir_sub.new_coordonate((x, y));

    //     let slot_add_one = self.find_value(add);
    //     let slot_sub_one = self.find_value(sub);

    //     add = dir_add.new_coordonate(add);
    //     sub = dir_sub.new_coordonate(sub);
        
    //     let slot_add_two = self.find_value(add);
    //     let slot_sub_two = self.find_value(sub);

    //     if slot_cmp_or![&Slot::Forbidden; [slot_add_one, slot_sub_one]]
    //         || slot_cmp_or![slot_enemy; [slot_add_one, slot_sub_one]]
    //     {
    //         return false;
    //     }
    //     add = dir_add.new_coordonate(add);
    //     sub = dir_sub.new_coordonate(sub);

    //     let slot_add_three = self.find_value(add);
    //     let slot_sub_three = self.find_value(sub);

    //     add = dir_add.new_coordonate(add);
    //     sub = dir_sub.new_coordonate(sub);

    //     let slot_add_four = self.find_value(add);
    //     let slot_sub_four = self.find_value(sub);

    //     let total_add = slot_cmp![slot_player; (slot_add_one, slot_add_two, slot_add_three, slot_add_four)];
    //     let total_sub = slot_cmp![slot_player; (slot_sub_one, slot_sub_two, slot_sub_three, slot_sub_four)];
		
    //     total_add + total_sub == 2 && (total_add == 2 || total_sub == 2 || slot_cmp_or![slot_player; [slot_add_one, slot_sub_one]])
    // }

    pub fn change_player_turn(&mut self)
    {
        self.turn += 1;
        match self.current_player
        {
            Player::One => self.current_player = Player::Two,
            _           => self.current_player = Player::One
        }
    }

    pub fn move_authorize(&self, x: i64, y: i64, dir: Direction) -> bool
    {
        self.is_available(dir.new_coordonate((x, y))) == 0
    }

    // pub fn number_captured(&mut self, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot), with_delete: bool) -> usize
    // {
    //     let mut count:usize = 0;

    //     for dir in Direction::iterator()
    //     {
    //         count += self.is_capture(dir, (x, y), (slot_player, slot_enemy), with_delete);
    //     }
    //     count
    // }

    // fn is_capture(&mut self, dir: &Direction, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot), with_delete: bool) -> usize
    // {
    //     if dir.next_three((x, y), self) == (slot_enemy, slot_enemy, slot_player)
    //     {
    //         if with_delete
    //         {
    //             dir.capture((x, y), self);
    //             match slot_player
    //             {
    //                 &Slot::PlayerOne => self.players_score.0 += 2,
    //                 _                => self.players_score.1 += 2
    //             }
    //             println!("Score: {:?}", self.players_score);
    //             if self.players_score.0 >= 10 || self.players_score.1 >= 10
    //             {
    //                 self.is_finish = true;
    //                 println!("Finish");
    //             }
    //         }
    //         2
    //     }
    //     else
    //     {
    //         0
    //     }
    // }

    // pub fn number_aligned(&self, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
    // {
    //     let mut count:usize = 0;

    //     for dir in Direction::iterator()
    //     {
    //         count += self.is_align(dir, (x, y), (slot_player, slot_enemy));
    //     }
    //     count
    // }

    // fn is_align(&self, dir: &Direction, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
    // {
    //     match dir.next_three((x, y), self)
    //     {
    //         (a, b, c) if (a, b, c) == (slot_player, slot_player, slot_player)  => 4,
    //         (a, b, c) if (a, b, c) == (slot_player, slot_player, &Slot::Empty) => 3,
    //         (a, b, c) if (a, b, c) == (&Slot::Empty, slot_player, slot_player) => 3,
    //         (a, b, c) if (a, b, c) == (slot_player, &Slot::Empty, slot_player) => 3,
    //         (a, b, _) if (a, b) == (slot_player, slot_player)                  => 2,
    //         (a, b, _) if (a, b) == (slot_player, &Slot::Empty)                 => 1,
    //         (a, b, _) if (a, b) == (&Slot::Empty, slot_player)                 => 1,
    //         _                                                                  => 0
    //     }
    // }

    // pub fn number_cut(&self, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
    // {
    //     let mut count:usize = 0;

    //     for dir in Direction::iterator()
    //     {
    //         count += self.is_cut(dir, (x, y), (slot_player, slot_enemy));
    //     }
    //     count
    // }

    // fn is_cut(&self, dir: &Direction, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
    // {
    //     match dir.next_three((x, y), self)
    //     {
    //         (a, b, c) if (a, b, c) == (slot_enemy, slot_enemy, slot_enemy)   => 4,
    //         (a, b, c) if (a, b, c) == (slot_enemy, slot_enemy, &Slot::Empty) => 3,
    //         (a, b, c) if (a, b, c) == (&Slot::Empty, slot_enemy, slot_enemy) => 3,
    //         (a, b, c) if (a, b, c) == (slot_enemy, &Slot::Empty, slot_enemy) => 3,
    //         (a, b, _) if (a, b) == (slot_enemy, slot_enemy)                  => 2,
    //         (a, b, _) if (a, b) == (slot_enemy, &Slot::Empty)                => 1,
    //         (a, b, _) if (a, b) == (&Slot::Empty, slot_enemy)                => 1,
    //         _                                                                => 0
    //     }
    // }

    pub fn find_value(&self, (x, y):(i64, i64)) -> i64
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return -1;
        }
        (self.value[y as usize] & (0o3 << (3 * x))) >> 3 * x
    }

    // pub fn is_winning_move(&self, (x, y):(i64, i64)) -> bool
    // {
    //     for axe in Direction::axes_iterator()
    //     {
    //         if self.is_five_align((x, y), find_slots_players!(self.current_player), axe)
    //         {
    //             println!("FIVE ALIGN FOR PLAYER {:?} !!!", find_slot_player!(self.current_player));
    //             return true;
    //         }
    //     }
    //     false
    // }

    // fn is_five_align(&self, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot), &(ref dir_add, ref dir_sub): &(Direction, Direction)) -> bool
    // {
    //     if self.is_capturable((x, y), (slot_player, slot_enemy))
    //     {
    //         return false;
    //     }

    //     let coord_add_one = dir_add.new_coordonate((x, y));
    //     let coord_sub_one = dir_sub.new_coordonate((x, y));

    //     let slot_add_one = self.find_value(coord_add_one);
    //     let slot_sub_one = self.find_value(coord_sub_one);

    //     let coord_add_two = dir_add.new_coordonate(coord_add_one);
    //     let coord_sub_two = dir_sub.new_coordonate(coord_sub_one);

    //     let slot_add_two = self.find_value(coord_add_two);
    //     let slot_sub_two = self.find_value(coord_sub_two);

    //     let coord_add_three = dir_add.new_coordonate(coord_add_two);
    //     let coord_sub_three = dir_sub.new_coordonate(coord_sub_two);

    //     let slot_add_three = self.find_value(coord_add_three);
    //     let slot_sub_three = self.find_value(coord_sub_three);

    //     let coord_add_four = dir_add.new_coordonate(coord_add_three);
    //     let coord_sub_four = dir_sub.new_coordonate(coord_sub_three);

    //     let slot_add_four = self.find_value(coord_add_four);
    //     let slot_sub_four = self.find_value(coord_sub_four);

    //     let total_add = slots_winning![slot_player; &slot_enemy; self; [(coord_add_one, slot_add_one), (coord_add_two, slot_add_two), (coord_add_three, slot_add_three), (coord_add_four, slot_add_four)]];
    //     let total_sub = slots_winning![slot_player; &slot_enemy; self; [(coord_sub_one, slot_sub_one), (coord_sub_two, slot_sub_two), (coord_sub_three, slot_sub_three), (coord_sub_four, slot_sub_four)]];

    //     // 4 because the current slot isn't taking in consideration in slot_winning! macro
    //     total_add + total_sub >= 4
    // }

    // pub fn is_capturable(&self, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot)) -> bool
    // {
    //     for &(ref dir_add, ref dir_sub) in Direction::axes_iterator()
    //     {
    //         let (slot_add_one, slot_add_two) = dir_add.next_two((x, y), self);
    //         let (slot_sub_one, slot_sub_two) = dir_sub.next_two((x, y), self);

    //         let is_capturable = match (slot_add_two, slot_add_one, slot_sub_one, slot_sub_two)
    //         {
    //             (a, b, c, _) if (a, b, c) == (&Slot::Empty, slot_player, slot_enemy) => true,
    //             (a, b, c, _) if (a, b, c) == (slot_enemy, slot_player, &Slot::Empty) => true,
    //             (_, b, c, d) if (b, c, d) == (&Slot::Empty, slot_player, slot_enemy) => true,
    //             (_, b, c, d) if (b, c, d) == (slot_enemy, slot_player, &Slot::Empty) => true,
    //             _                                                                    => false
    //         };
    //         if is_capturable
    //         {
    //             return true;
    //         }
    //     }
    //     false
    // }

    // pub fn align_value(&self, dir: &Direction, (x, y):(i64, i64), (slot_player, slot_enemy): (&Slot, &Slot), empty_before: bool) -> (i64, i64)
    // {
    //     // add check is_wining move for 4 or 5 slot
    //     match dir.next_four((x, y), self)
    //     {
    //         (a, b, c, d) if (a, b, c, d) == (slot_player, slot_player, slot_player, slot_player)                  => (100, 4),
    //         (a, b, c, d) if (a, b, c, d) == (slot_player, slot_player, slot_player, &Slot::Empty) && empty_before => (80, 3),
    //         (a, b, c, d) if (a, b, c, d) == (slot_player, slot_player, slot_player, &Slot::Empty)                 => (20, 3),
    //         (a, b, c, d) if (a, b, c) == (slot_player, slot_player, slot_player) && empty_before                  => (20, 3),
    //         (a, b, c, d) if (a, b, c, d) == (slot_player, slot_player, slot_player, &Slot::Empty)                 => (20, 3),
    //         (a, b, c, d) if (a, b, c, d) == (slot_player, slot_player, &Slot::Empty, slot_player)                 => (15, 4),
    //         (a, b, c, d) if (a, b, c, d) == (&Slot::Empty, slot_player, slot_player, slot_player)                 => (5, 5),
    //         (a, b, c, _) if (a, b, c) == (slot_enemy, &Slot::Empty, slot_enemy)                 => (3, 1),
    //         (a, b, c, _) if (a, b, c) == (slot_player, slot_player, &Slot::Empty)                                 => (4, 2),
    //         (a, b, c, _) if (a, b, c) == (slot_player, &Slot::Empty, slot_player)                                 => (4, 3),
    //         (a, b, c, _) if (a, b, c) == (&Slot::Empty, slot_player, slot_player) && empty_before                 => (4, 3),
    //         (a, b, c, _) if (a, b, c) == (slot_enemy, slot_enemy, slot_player)                                   => (15, 3),
    //         (a, b, c, _) if (a, b, c) == (slot_enemy, slot_enemy, &Slot::Empty)                                   => (10, 2),
    //         (a, b, _, _) if (a, b) == (slot_enemy, slot_enemy)                                   => (4, 2),
    //         (a, b, _, _) if (a, b) == (slot_player, slot_enemy) && empty_before                                   => (-15, 2),
    //         (a, b, c, d) if (a, b, c, d) == (&Slot::Empty, slot_player, slot_player, slot_player)                 => (4, 4),

    //         (a, b, c, d) if (a, b, c, d) == (slot_player, &Slot::Empty, slot_player, &Slot::Empty)                => (3, 3),
    //         (a, b, c, d) if (a, b, c, d) == (slot_player, slot_player, &Slot::Empty, &Slot::Empty)                => (3, 2),
    //         (a, b, c, _) if (a, b, c) == (slot_player, &Slot::Empty, &Slot::Empty) && empty_before                => (2, 1),
    //         (a, b, _, _) if (a, b) == (&Slot::Empty, slot_player)                                                 => (2, 2),
    //         _                                                                                                     => (0, 1)
    //     }
    // }

    pub fn print_map(&self) -> ()
    {
        for y in &self.value
        {
            for x in 0..19
            {
                match ((y & (0o3 << (3 * (18 - x)))) >> 3 * (18 - x))
                {
                    1 => print!("1 "),
                    2 => print!("2 "),
                    _ => print!("- ")
                }
            }
            print!("\n");
        }
    }
}
