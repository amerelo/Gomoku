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

const DTHREE_MOVE_P1: [(i128, i128, i128); 9] = [
                                             (0o3333333333, 0o001010000, 6),
                                             (0o3333333333, 0o001000100, 12),
                                             (0o3333333333, 0o000010100, 18),
                                             (0o33333333333, 0o00100010000, 6),
                                             (0o33333333333, 0o00100000100, 12),
                                             (0o33333333333, 0o00000010100, 24),
                                             (0o33333333333, 0o00101000000, 6),
                                             (0o33333333333, 0o00100000100, 18),
                                             (0o33333333333, 0o00001000100, 24)
                                            ];

const DTHREE_MOVE_P2: [(i128, i128, i128); 9] = [
                                             (0o3333333333, 0o002020000, 6),
                                             (0o3333333333, 0o002000200, 12),
                                             (0o3333333333, 0o000020200, 18),
                                             (0o33333333333, 0o00200020000, 6),
                                             (0o33333333333, 0o00200000200, 12),
                                             (0o33333333333, 0o00000020200, 24),
                                             (0o33333333333, 0o00202000000, 6),
                                             (0o33333333333, 0o00200000200, 18),
                                             (0o33333333333, 0o00002000200, 24)
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
                        insert_without_double![((y as i128 - 1), x as i128), area];
                        insert_without_double![((y as i128 - 1), (x as i128 - 1)), area];
                        insert_without_double![((y as i128 - 1), (x + 1) as i128), area];
						insert_without_double![((y + 1) as i128, (x as i128 - 1)), area];
						insert_without_double![((y + 1) as i128, (x + 1) as i128), area];
						insert_without_double![((y + 1) as i128, x as i128), area];
						insert_without_double![(y as i128, (x as i128 - 1)), area];
						insert_without_double![(y as i128, (x + 1) as i128), area];
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

    pub fn move_authorize(&self, x: i128, y: i128, dir: Direction) -> bool
    {
        self.is_available(dir.new_coordonate((x, y))) == 0
    }

    pub fn find_value(&self, (x, y):(i128, i128)) -> i128
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return -1;
        }
        (self.value[y as usize] & (0o3 << (3 * x))) >> 3 * x
    }

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
