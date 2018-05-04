use goban::player::{Player, PlayerKind};
use goban::direction::{Direction};
use std::i128;

const  SIZEMAP: usize = 19;
const  RSIZEMAP: i128 = 18;
// const  CMP_THREE_MOVE_P2: [i128; 3] = [0o02220, 0o022020, 0o020220];
// const  CMP_THREE_MOVE_D_P1: [i128; 3] = [0o001010100, 0o000101000100, 0o00100010100];
// const  CMP_THREE_MOVE_D_P2: [i128; 3] = [0o002020200, 0o000202000200, 0o00200020200];

// const  THREE_MOVE_P2: [i128; 3] = [0o32223, 0o322323, 0o323223];
// const  THREE_MOVE_D_P1: [i128; 3] = [0o331313133, 0o333131333133, 0o33133313133];
// const  THREE_MOVE_D_P2: [i128; 3] = [0o332323233, 0o333232333233, 0o33233323233];

const THREE_MOVE_P1: [(i128, i128, i128); 9] = [
                                             (0o33333, 0o01100, 3),
                                             (0o33333, 0o01010, 6),
                                             (0o33333, 0o00110, 9),
                                             (0o333333, 0o10100, 3),
                                             (0o333333, 0o010010, 6),
                                             (0o333333, 0o000110, 12),
                                             (0o333333, 0o011000, 3),
                                             (0o333333, 0o010010, 9),
                                             (0o333333, 0o001010, 12)
                                            ];

const THREE_MOVE_P2: [(i128, i128, i128); 9] = [
                                             (0o33333, 0o02200, 3),
                                             (0o33333, 0o02020, 6),
                                             (0o33333, 0o00220, 9),
                                             (0o333333, 0o20200, 3),
                                             (0o333333, 0o020020, 6),
                                             (0o333333, 0o000220, 12),
                                             (0o333333, 0o022000, 3),
                                             (0o333333, 0o020020, 9),
                                             (0o333333, 0o002020, 12)
                                            ];

#[derive(Debug, Clone)]
pub struct Map
{
    pub value: Vec<i128>,
    pub value_rotate: Vec<i128>,
    pub value_diagonale: Vec<i128>,
    pub value_diagonale_rotate: Vec<i128>,
    pub players_kind: (PlayerKind, PlayerKind),
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
            value: mapinit![SIZEMAP, 0 as i128],
            value_rotate: mapinit![SIZEMAP, 0 as i128],
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

    pub fn set_value(&mut self, (x, y):(i128, i128), value: i128) -> ()
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

	pub fn area_of_interest(&self) -> Vec<(i128, i128)>
	{
		let mut area: Vec<(i128, i128)> = vec![];
		// let mask:i128 = 0o3333_333333_333333_333;
		let size_map: i128 = 18;

		for (y, elem_y) in self.value.iter().enumerate()
		{
			if *elem_y != 0
			{
				for x in 0..19
				{
					if ((elem_y >> ((size_map - x) * 3)) & 0x3 ) != 0
					{
                        area.push(((y as i128 - 1), x as i128));
                        area.push(((y as i128 - 1), (x as i128 - 1)));
                        area.push(((y as i128 - 1), (x + 1) as i128));
						area.push(((y + 1) as i128, (x as i128 - 1)));
						area.push(((y + 1) as i128, (x + 1) as i128));
						area.push(((y + 1) as i128, x as i128));
						area.push((y as i128, (x as i128 - 1)));
						area.push((y as i128, (x + 1) as i128));
					}
				}	
			}
		}
		area
	}

    pub fn is_available(&self, (x, y):(i128, i128)) -> i128
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

    fn is_double_three_move(&self, (x, y):(i128, i128)) -> i128
    {
        match self.three_move_number((x, y), find_tm_player![self.current_player, THREE_MOVE_P1, THREE_MOVE_P2])
        {
            1 => 0,
            0 => 0,
            _ => -1
        }
    }

    fn three_move_number(&self, (x, y):(i128, i128), slot_player: [(i128, i128, i128); 9]) -> usize
    {
        let mut count:usize = 0;

        count += match x
        {
            18 => 0,
            17 => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_player; [0, 3, 6]] as usize,
            16 => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_player; [0, 1, 3, 4]] as usize,
            15 => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_player; [0, 1, 2, 3, 4, 7]] as usize,
            _  => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_player; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
        };

        count += match y
        {
            0 => 0,
            1 => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_player; [0, 3, 6]] as usize,
            2 => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_player; [0, 1, 3, 4]] as usize,
            3 => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_player; [0, 1, 2, 3, 4, 7]] as usize,
            _ => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_player; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
        };


        // count += match y
        // {
        //     0 => 0,
        //     1 => slot_cmp![self.value_rotate[x as usize], y * 3 ; THREE_MOVE_P1; [0]] as usize,
        //     2 => slot_cmp![self.value_rotate[x as usize], y * 3 ; THREE_MOVE_P1; [0, 1]] as usize,
        //     _  => slot_cmp![self.value_rotate[x as usize], y * 3 ; THREE_MOVE_P1; [0, 1, 2]] as usize
        // };

        // count += slot_cmp![self.value_rotate[x as usize], y + 1; THREE_MOVE_P1; [0, 1, 2]] as usize;
        // let conv:(i128, i128) = match x >= y
        // {
        //     true => (18 + (x - y) as i128, (x + y)as i128), 
        //     _    => (18 - (y - x) as i128, (x + y)as i128)
        // };

        // count += slot_cmp![self.value[y as usize], 0; THREE_MOVE_P1; [0, 1, 2]] as usize;
        // println!("value {:o}", self.value[y as usize]);
        // println!("count {}", count);
        // println!("rotat {:o}", self.value_rotate[x as usize]);
        // println!("count {} {} {} {:o} {:o} {}", count, x, y, self.value[y as usize], ((self.value[y as usize] & (THREE_MOVE_P1[0].0 << ((RSIZEMAP - x) * 3 ))) >> ((RSIZEMAP - x) * 3 )), ((self.value[y as usize] & (THREE_MOVE_P1[0].0 << ((RSIZEMAP - x - 1) * 3 ))) >> ((RSIZEMAP - x - 1) * 3 )) == THREE_MOVE_P1[0].1 );
        count
    }

    // fn is_free_three(&self, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot), &(ref dir_add, ref dir_sub): &(Direction, Direction)) -> bool
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

    pub fn move_authorize(&self, x: i128, y: i128, dir: Direction) -> bool
    {
        self.is_available(dir.new_coordonate((x, y))) == 0
    }

    // pub fn number_captured(&mut self, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot), with_delete: bool) -> usize
    // {
    //     let mut count:usize = 0;

    //     for dir in Direction::iterator()
    //     {
    //         count += self.is_capture(dir, (x, y), (slot_player, slot_enemy), with_delete);
    //     }
    //     count
    // }

    // fn is_capture(&mut self, dir: &Direction, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot), with_delete: bool) -> usize
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

    // pub fn number_aligned(&self, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
    // {
    //     let mut count:usize = 0;

    //     for dir in Direction::iterator()
    //     {
    //         count += self.is_align(dir, (x, y), (slot_player, slot_enemy));
    //     }
    //     count
    // }

    // fn is_align(&self, dir: &Direction, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
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

    // pub fn number_cut(&self, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
    // {
    //     let mut count:usize = 0;

    //     for dir in Direction::iterator()
    //     {
    //         count += self.is_cut(dir, (x, y), (slot_player, slot_enemy));
    //     }
    //     count
    // }

    // fn is_cut(&self, dir: &Direction, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot)) -> usize
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

    pub fn find_value(&self, (x, y):(i128, i128)) -> i128
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return -1;
        }
        (self.value[y as usize] & (0o3 << (3 * x))) >> 3 * x
    }

    // pub fn is_winning_move(&self, (x, y):(i128, i128)) -> bool
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

    // fn is_five_align(&self, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot), &(ref dir_add, ref dir_sub): &(Direction, Direction)) -> bool
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

    // pub fn is_capturable(&self, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot)) -> bool
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

    // pub fn align_value(&self, dir: &Direction, (x, y):(i128, i128), (slot_player, slot_enemy): (&Slot, &Slot), empty_before: bool) -> (i128, i128)
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
                match (y & (0o3 << (3 * (18 - x)))) >> 3 * (18 - x)
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
