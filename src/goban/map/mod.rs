pub mod constant;
use goban::player::{Player, PlayerKind};
use std::i128;
use heuristic;
use goban::map::constant::{*};
use goban::finish::{Finish};

#[derive(Debug, Clone)]
pub struct Map
{
	pub value: Vec<i128>,
	pub value_rotate: Vec<i128>,
	pub value_diagonale: Vec<i128>,
	pub value_diagonale_rotate: Vec<i128>,
	pub players_kind: (PlayerKind, PlayerKind),
	pub players_score: (i128, i128),
	pub current_player: Player,
	pub turn: usize,
	pub is_finish: Finish,
}

impl Default for Map
{
	fn default() -> Map
	{
		Map
		{
			value: mapinit![SIZEMAP, 0 as i128],
			value_rotate: mapinit![SIZEMAP, 0 as i128],
			value_diagonale: map_diagonale_init![SIZEMAP * 2, 0o3333333333333333333333333333333333333 as i128],
			value_diagonale_rotate: map_diagonale_init![SIZEMAP * 2, 0o3333333333333333333333333333333333333 as i128],
			players_kind: (PlayerKind::Human, PlayerKind::Human),
			players_score: (0, 0),
			current_player: Player::One,
			turn: 1,
			is_finish: Finish::None,
		}
	}
}

impl Map
{
    pub fn reset(&mut self)
    {
        self.value = mapinit![SIZEMAP, 0];
        self.value_rotate = mapinit![SIZEMAP, 0];
        self.value_diagonale = map_diagonale_init![SIZEMAP * 2, 0o3333333333333333333333333333333333333 as i128];
        self.value_diagonale_rotate = map_diagonale_init![SIZEMAP * 2, 0o3333333333333333333333333333333333333 as i128];
        self.players_score = (0, 0);
        self.current_player = Player::One;
        self.is_finish = Finish::None;
        self.turn = 1;
    }

	pub fn reset_players(&mut self, player1: PlayerKind, player2: PlayerKind)
	{
		self.players_kind = (player1, player2);
	}

    pub fn set_value(&mut self, (x, y):(i128, i128), value: i128) -> ()
    {
        self.value[y as usize] ^= value << (3 * (RSIZEMAP - x));
        self.value_rotate[x as usize] ^= value << 3 * y;
        let conv:(i128, i128) = match x >= y
        {
            true => (RSIZEMAP + (x - y) as i128, (x + y)as i128), 
            _    => (RSIZEMAP - (y - x) as i128, (x + y)as i128)
        };
        self.value_diagonale[conv.1 as usize] ^= (value as i128) << 3 * conv.0;
        self.value_diagonale_rotate[conv.0 as usize] ^= (value as i128) << 3 * ((RSIZEMAP as i128) * 2 - conv.1);
    }

	pub fn area_of_interest(&self, number: usize, player: &Player) -> Vec<(i128, i128, i128)>
	{
		let mut area: Vec<(i128, i128, i128)> = vec![];

		for (y, elem_y) in self.value.iter().enumerate()
		{
			if *elem_y != 0
			{
				for x in 0..SIZEMAP
				{
					if ((elem_y >> ((RSIZEMAP - x) * 3)) & 0x3 ) != 0
					{
						insert_without_double![((y as i128 - 1), x as i128, 0), area, self, player];
						insert_without_double![((y as i128 - 1), (x as i128 - 1), 0), area, self, player];
						insert_without_double![((y as i128 - 1), (x + 1) as i128, 0), area, self, player];
						insert_without_double![((y + 1) as i128, (x as i128 - 1), 0), area, self, player];
						insert_without_double![((y + 1) as i128, (x + 1) as i128, 0), area, self, player];
						insert_without_double![((y + 1) as i128, x as i128, 0), area, self, player];
						insert_without_double![(y as i128, (x as i128 - 1), 0), area, self, player];
						insert_without_double![(y as i128, (x + 1) as i128, 0), area, self, player];
					}
				}
			}
		}
		if area.len() == 0
		{
			area.push((9, 9, 0));
		}

		for t in &mut area
		{
			t.2 = heuristic::value_slot(self, *t, player);
		}
		area.sort_by_key(|k| -k.2);
		if number < area.len()
		{
			area[0 .. number].to_vec()
		}
		else
		{
			area
		}
	}

    pub fn is_available(&self, (x, y):(i128, i128), player: &Player) -> i128
    {
        if x > RSIZEMAP || y > RSIZEMAP || x < 0 || y < 0
        {
            return -1;
        }
        match (self.value[y as usize] & 0o3 << (3 * (RSIZEMAP - x))) >> 3 * (RSIZEMAP - x)
        {
            0     => self.is_double_three_move((x, y), player),
            val   => val
        }
    }

    fn is_double_three_move(&self, (x, y):(i128, i128), player: &Player) -> i128
    {
        match self.three_move_number((x, y), find_tm_player![player, (THREE_MOVE_P1, DTHREE_MOVE_P1), (THREE_MOVE_P2, DTHREE_MOVE_P2)])
        {
            0 | 1 => 0,
            _     => -1
        }
    }

     pub fn number_captured(&mut self, (x, y):(i128, i128), slot_player: i128, with_delete: bool) -> usize
     {
        let conv:(i128, i128) = match x >= y
        {
            true => (RSIZEMAP + (x - y) as i128, (x + y)as i128), 
            _    => (RSIZEMAP - (y - x) as i128, (x + y)as i128)
        };
        
        self.is_capture((x, y, conv.0, conv.1), slot_player, with_delete)
     }

    fn conv_xy(&self, (x, y): (i128, i128)) -> (i128, i128)
    {
        match x >= y
        {
            true => { return (RSIZEMAP + (x - y) as i128, (x + y) as i128) },
            _    => { return (RSIZEMAP - (y - x) as i128, (x + y) as i128) }
        };
    }

    fn all_conv_xy(&self, (x, y): (i128, i128)) -> (i128, i128, i128, i128)
    {
        let conv:(i128, i128) = self.conv_xy((x, y));
        (x, y, conv.0, conv.1)
    }

    fn is_capture(&mut self, (x, y, x2, y2):(i128, i128, i128, i128), slot_player: i128, with_delete: bool) -> usize
    {
       let masks = find_tm_player![self.current_player, CAPTURE_P1, CAPTURE_P2];
       let mut count:usize = 0;

       if x >= 2 && slot_capture![self.value[y as usize], 3 * (RSIZEMAP - x); masks; 0]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x - 1, y, x - 2, y));
           }
       }
       if x <= RSIZEMAP - 3 && slot_capture![self.value[y as usize], 3 * (RSIZEMAP - x); masks; 1]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x + 1, y, x + 2, y));
           }
       }

       if y >= 0 && slot_capture![self.value_rotate[x as usize], 3 * y; masks; 0]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x, y + 1, x, y + 2));
           }
       }

       if y >= 3 && slot_capture![self.value_rotate[x as usize], 3 * y; masks; 1]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x, y - 1, x, y - 2));
           }
       }
       if x <= RSIZEMAP - 2 && y >= 2 && slot_capture![self.value_diagonale[y2 as usize], 3 * x2; masks; 2]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x + 1, y - 1, x + 2, y - 2));
           }
       }

       if y <= RSIZEMAP - 3 && x >= 3 && slot_capture![self.value_diagonale[y2 as usize], 3 * x2; masks; 3]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x - 1, y + 1, x - 2, y + 2));
           }
       }

       if x >= 2 && y >= 2 && slot_capture![self.value_diagonale_rotate[x2 as usize], 3 * (RSIZEMAP * 2 - y2); masks; 2]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x - 1, y - 1, x - 2, y - 2));
           }
       }

       if x <= RSIZEMAP - 3 && y <= RSIZEMAP - 3 && slot_capture![self.value_diagonale_rotate[x2 as usize], 3 * (RSIZEMAP * 2 - y2); masks; 3]
       {
           count += 2;
           if with_delete
           {
               self.delete_captured(slot_player, (x + 1, y + 1, x + 2, y + 2));
           }
       }
       count
    }

    fn delete_captured(&mut self, slot_player: i128, (x, y, x2, y2):(i128, i128, i128, i128)) -> ()
    {
        let slot_enemy = find_slot_enemy![self.current_player];

        match slot_player
        {
            1 => self.players_score.0 += 2,
            _ => self.players_score.1 += 2
        }
        self.set_value((x, y), slot_enemy);
        self.set_value((x2, y2), slot_enemy);

		if self.players_score.0 >= 10 || self.players_score.1 >= 10
        {
            match slot_player
            {
                1 => { self.is_finish = Finish::CapturePlayerOne },
                _ => { self.is_finish = Finish::CapturePlayerTwo }
            }
        }
    }

    pub fn five_align(&mut self) -> ()
    {
        let mask = 0o33333;
        let d_mask = 0o303030303;

        for (y, elem_y) in self.value.iter().enumerate()
        {
            if *elem_y != 0
            {
                for x in 4..SIZEMAP
                {
                    let value = (elem_y >> ((RSIZEMAP - x) * 3)) & mask;
                    if value == 0o11111 && !self.is_capturable(self.all_conv_xy((x, y as i128)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x - 1, y as i128)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x - 2, y as i128)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x - 3, y as i128)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x - 4, y as i128)), &Player::One)
                    {
                        self.is_finish = Finish::AlignPlayerOne;
                        return ;
                    }
                    else if value == 0o22222 && !self.is_capturable(self.all_conv_xy((x, y as i128)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x - 1, y as i128)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x - 2, y as i128)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x - 3, y as i128)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x - 4, y as i128)), &Player::Two)
                    {
                        self.is_finish = Finish::AlignPlayerTwo;
                        return ;
                    }
                }
            }
        }
        for (x, elem_y) in self.value_rotate.iter().enumerate()
        {
            if *elem_y != 0
            {
                for y in 4..SIZEMAP
                {
                    let value = (elem_y >> ((RSIZEMAP - y) * 3)) & mask;
                    if value == 0o11111 && !self.is_capturable(self.all_conv_xy((x as i128, y)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x as i128, y - 1)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x as i128, y - 2)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x as i128, y - 3)), &Player::One)
                                        && !self.is_capturable(self.all_conv_xy((x as i128, y - 4)), &Player::One)
                    {
                        self.is_finish = Finish::AlignPlayerOne;
                        return ;
                    }
                    else if value == 0o22222 && !self.is_capturable(self.all_conv_xy((x as i128, y)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x as i128, y - 1)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x as i128, y - 2)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x as i128, y - 3)), &Player::Two)
                                             && !self.is_capturable(self.all_conv_xy((x as i128, y - 4)), &Player::Two)
                    {
                        self.is_finish = Finish::AlignPlayerTwo;
                        return ;
                    }
                }
            }
        }

        for (y, elem_y) in self.value_diagonale.iter().enumerate()
        {
            if *elem_y != 0
            {
                for x in 0..(SIZEMAP * 2)
                {
                    let value = (elem_y >> (x * 3)) & d_mask;
                    if value == 0o101010101
                    {
                        let (real_x, real_y) = self.find_value((x, y as i128), false);
                        if !self.is_capturable(self.all_conv_xy((real_x, real_y)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x + 1, real_y - 1)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x + 2, real_y - 2)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x + 3, real_y - 3)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x + 4, real_y - 4)), &Player::One)
                        {
                            self.is_finish = Finish::AlignPlayerOne;
                            return ;
                        }
                    }
                    else if value == 0o202020202
                    {
                        let (real_x, real_y) = self.find_value((x, y as i128), false);
                        if !self.is_capturable(self.all_conv_xy((real_x, real_y)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x + 1, real_y - 1)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x + 2, real_y - 2)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x + 3, real_y - 3)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x + 4, real_y - 4)), &Player::Two)
                        {
                            self.is_finish = Finish::AlignPlayerTwo;
                            return ;
                        }
                    }
                }
            }
        }

        for (y, elem_y) in self.value_diagonale_rotate.iter().enumerate()
        {
            if *elem_y != 0
            {
                for x in 0..(SIZEMAP * 2)
                {
                    let value = (elem_y >> (x * 3)) & d_mask;
                    if value == 0o101010101
                    {
                        let (real_x, real_y) = self.find_value((x, y as i128), true);
                        if !self.is_capturable(self.all_conv_xy((real_x, real_y)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x - 1, real_y - 1)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x - 2, real_y - 2)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x - 3, real_y - 3)), &Player::One)
                            && !self.is_capturable(self.all_conv_xy((real_x - 4, real_y - 4)), &Player::One)
                        {
                            self.is_finish = Finish::AlignPlayerOne;
                            return ;
                        }
                    }
                    else if value == 0o202020202
                    {
                        let (real_x, real_y) = self.find_value((x, y as i128), true);
                        if !self.is_capturable(self.all_conv_xy((real_x, real_y)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x - 1, real_y - 1)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x - 2, real_y - 2)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x - 3, real_y - 3)), &Player::Two)
                            && !self.is_capturable(self.all_conv_xy((real_x - 4, real_y - 4)), &Player::Two)
                        {
                            self.is_finish = Finish::AlignPlayerTwo;
                            return ;
                        }
                    }
                }
            }
        }
    }

    fn is_capturable(&self, (x, y, x2, y2):(i128, i128, i128, i128), slot_player: &Player) -> bool
    {
       let masks = find_tm_player![slot_player, IS_CAPTURABLE_P1, IS_CAPTURABLE_P2];

       if x < 0 || x > RSIZEMAP || y < 0 || y > RSIZEMAP
       {
           return false;
       }
       if x >= 2 && slot_capturable![self.value[y as usize], 3 * (RSIZEMAP - x); masks; [0, 2]]
       {
           return true;
       }
       if x <= RSIZEMAP - 3 && slot_capturable![self.value[y as usize], 3 * (RSIZEMAP - x); masks; [1, 3]]
       {
           return true;
       }
       if y >= 0 && slot_capturable![self.value_rotate[x as usize], 3 * y; masks; [0, 2]]
       {
           return true;
       }
       if y >= 3 && slot_capturable![self.value_rotate[x as usize], 3 * y; masks; [1, 3]]
       {
           return true;
       }
       if x <= RSIZEMAP - 2 && y >= 2 && slot_capturable![self.value_diagonale[y2 as usize], 3 * x2; masks; [4, 6]]
       {
           return true;
       }
       if y <= RSIZEMAP - 3 && x >= 3 && slot_capturable![self.value_diagonale[y2 as usize], 3 * x2; masks; [5, 7]]
       {
           return true;
       }
       if x >= 2 && y >= 2 && slot_capturable![self.value_diagonale_rotate[x2 as usize], 3 * (RSIZEMAP * 2 - y2); masks; [4, 6]]
       {
           return true;
       }
       if x <= RSIZEMAP - 3 && y <= RSIZEMAP - 3 && slot_capturable![self.value_diagonale_rotate[x2 as usize], 3 * (RSIZEMAP * 2 - y2); masks; [5, 7]]
       {
           return true;
       }
       false
    }

    fn three_move_number(&self, (x, y):(i128, i128), (slot_hv, slot_d): ([(i128, i128, i128); 9], [(i128, i128, i128); 9])) -> usize
    {
        let mut count:usize = 0;

        if x == 0 || y == 0 || x == RSIZEMAP || y == RSIZEMAP
        {
            return count;
        }

        count += match x
        {
            17 => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_hv; [0, 3, 6]] as usize,
            16 => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_hv; [0, 1, 3, 4]] as usize,
            15 => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_hv; [0, 1, 2, 3, 4, 7]] as usize,
            _  => slot_cmp![self.value[y as usize], (RSIZEMAP - x) * 3 ; slot_hv; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
        };

        count += match y
        {
            1 => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_hv; [0, 3, 6]] as usize,
            2 => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_hv; [0, 1, 3, 4]] as usize,
            3 => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_hv; [0, 1, 2, 3, 4, 7]] as usize,
            _ => slot_cmp![self.value_rotate[x as usize], y * 3 ; slot_hv; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
        };

       let conv:(i128, i128) = self.conv_xy((x, y));

        if conv.1 < 3 || conv.1 > 33 || conv.0 < 3 || conv.0 > 33
        {
            return count;
        }

        count += match conv.0
        {
            3 ... 5 => slot_cmp![self.value_diagonale[conv.1 as usize], conv.0 * 3 ; slot_d; [0, 3, 6]] as usize,
            6 | 7 => slot_cmp![self.value_diagonale[conv.1 as usize], conv.0 * 3 ; slot_d; [0, 1, 2, 3, 4, 6, 7]] as usize,
            _ => slot_cmp![self.value_diagonale[conv.1 as usize], conv.0 * 3 ; slot_d; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
        };

        count += match (RSIZEMAP as i128) * 2 - conv.1
        {
            3 ... 5 => slot_cmp![self.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; slot_d; [0, 3, 6]] as usize,
            6 | 7 => slot_cmp![self.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; slot_d; [0, 1, 2, 3, 4, 6, 7]] as usize,
            _ => slot_cmp![self.value_diagonale_rotate[conv.0 as usize], ((RSIZEMAP as i128) * 2 - conv.1) * 3 ; slot_d; [0, 1, 2, 3, 4, 5, 7, 8]] as usize
        };

        count
    }

    pub fn change_player_turn(&mut self)
    {
        match self.current_player
        {
            Player::One => self.current_player = Player::Two,
            _           => { self.turn += 1; self.current_player = Player::One }
        }
    }

    fn find_value(&self, (ref_x, ref_y):(i128, i128), is_rotate: bool) -> (i128, i128)
    {
        for y in 0..SIZEMAP
        {
            for x in 0..SIZEMAP
            {
                let conv:(i128, i128) = self.conv_xy((x, y));

                if !is_rotate && conv.0 == ref_x && conv.1 == ref_y
                {
                    return (x, y);
                }
                else if is_rotate && conv.0 == ref_x && conv.1 == ref_y
                {
                    return (y, RSIZEMAP - x);
                }
            }
        }
        (-1, -1)
    }

    pub fn print_map(&self) -> ()
    {
        for y in &self.value
        {
            for x in 0..SIZEMAP
            {
                match (y & (0o3 << (3 * (RSIZEMAP - x)))) >> 3 * (RSIZEMAP - x)
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
