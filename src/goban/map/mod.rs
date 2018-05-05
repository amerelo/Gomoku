pub mod constant;
use goban::player::{Player, PlayerKind};
use goban::direction::{Direction};
use std::i128;
use heuristic;
use goban::map::constant::{SIZEMAP, RSIZEMAP, THREE_MOVE_P1, THREE_MOVE_P2, DTHREE_MOVE_P1, DTHREE_MOVE_P2};

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
            true => (RSIZEMAP + (x - y) as i128, (x + y)as i128), 
            _    => (RSIZEMAP - (y - x) as i128, (x + y)as i128)
        };
        self.value_diagonale[conv.1 as usize] ^= (value as i128) << 3 * conv.0;
        self.value_diagonale_rotate[conv.0 as usize] ^= (value as i128) << 3 * ((RSIZEMAP as i128) * 2 - conv.1);
    }

	pub fn area_of_interest(&self, number: usize) -> Vec<(i128, i128, i128)>
	{
		let mut area: Vec<(i128, i128, i128)> = vec![];
		// let mask:i128 = 0o3333_333333_333333_333;
		let size_map: i128 = RSIZEMAP;

		for (y, elem_y) in self.value.iter().enumerate()
		{
			if *elem_y != 0
			{
				for x in 0..SIZEMAP
				{
					if ((elem_y >> ((size_map - x) * 3)) & 0x3 ) != 0
					{
                        insert_without_double![((y as i128 - 1), x as i128, 0), area];
                        insert_without_double![((y as i128 - 1), (x as i128 - 1), 0), area];
                        insert_without_double![((y as i128 - 1), (x + 1) as i128, 0), area];
						insert_without_double![((y + 1) as i128, (x as i128 - 1), 0), area];
						insert_without_double![((y + 1) as i128, (x + 1) as i128, 0), area];
						insert_without_double![((y + 1) as i128, x as i128, 0), area];
						insert_without_double![(y as i128, (x as i128 - 1), 0), area];
						insert_without_double![(y as i128, (x + 1) as i128, 0), area];
					}
				}
			}
		}
		for t in &mut area
        {
            t.2 = heuristic::value_slot(self, *t);
        }
        area.sort_by_key(|k| -k.2);
        if number < area.len() - 1
        {
            area[0 .. number].to_vec()
        }
        else
        {
            area
        }
    }

    pub fn is_available(&self, (x, y):(i128, i128)) -> i128
    {
        if x > RSIZEMAP || y > RSIZEMAP || x < 0 || y < 0
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
        match self.three_move_number((x, y), find_tm_player![self.current_player, (THREE_MOVE_P1, DTHREE_MOVE_P1), (THREE_MOVE_P2, DTHREE_MOVE_P2)])
        {
            1 => 0,
            0 => 0,
            _ => -1
        }
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

       let conv:(i128, i128) = match x >= y
        {
            true => (RSIZEMAP + (x - y) as i128, (x + y)as i128),
            _    => (RSIZEMAP - (y - x) as i128, (x + y)as i128)
        };

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

        // println!("rotat {:o}", count);
        count
    }

    pub fn change_player_turn(&mut self)
    {
        self.turn += 1;
        match self.current_player
        {
            Player::One => self.current_player = Player::Two,
            _           => self.current_player = Player::One
        }
    }

    pub fn find_value(&self, (x, y):(i128, i128)) -> i128
    {
        if x > RSIZEMAP || y > RSIZEMAP || x < 0 || y < 0
        {
            return -1;
        }
        (self.value[y as usize] & (0o3 << (3 * x))) >> 3 * x
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
