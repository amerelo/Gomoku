use goban::player::{Player, PlayerKind};
use goban::direction::{Direction};
// use std::ops::Add;

const  SIZEMAP: usize = 19;

#[derive(Debug, PartialEq)]
pub enum Slot
{
    PlayerOne,
    PlayerTwo,
    Empty,
    Forbidden,
}

#[derive(Debug, PartialEq)]
pub enum HintSlot
{
    Used,
    Empty,
    CapturePlayerOne,
    CapturePlayerTwo,
}

#[derive(Debug)]
pub struct Map
{
    pub value: Vec<Vec<Slot>>,
    pub hint_map: Vec<Vec<HintSlot>>,
    pub players_kind: (PlayerKind, PlayerKind), // easier way for handling players number and players kind
    pub players_score: (usize, usize),
    pub current_player: Player,
    pub is_finish: bool,
}

impl Default for Map
{
    fn default() -> Map
    {
        Map
        {
            value: mapinit![SIZEMAP, Slot::Empty],
            hint_map: mapinit![SIZEMAP, HintSlot::Empty],
            players_kind: (PlayerKind::Human, PlayerKind::Human),
            players_score: (0, 0),
            current_player: Player::One,
            is_finish: false,
        }
    }
}

impl Map
{
    pub fn reset(&mut self) -> ()
    {
        self.value = mapinit![SIZEMAP, Slot::Empty];
        self.hint_map = mapinit![SIZEMAP, HintSlot::Empty];
        self.players_score = (0, 0);
        self.current_player = Player::One;
        self.is_finish = false;
    }

    pub fn is_available(&self, (x, y):(i32, i32)) -> Slot
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return Slot::Forbidden;
        }
        match self.value[y as usize][x as usize]
        {
            Slot::Empty     => self.is_double_three_move((x, y)),
            Slot::PlayerOne => Slot::PlayerOne,
            _               => Slot::PlayerTwo
        }
    }

    pub fn change_player_turn(&mut self)
    {
        match self.current_player
        {
            Player::One => self.current_player = Player::Two,
            _           => self.current_player = Player::One
        }
    }

    pub fn move_authorize(&self, x: i32, y: i32, dir: Direction) -> bool
    {
        self.is_available(dir.new_coordonate((x, y))) == Slot::Empty
    }

    pub fn number_captured(&mut self, (x, y):(i32, i32), with_delete: bool) -> usize
    {
        let mut count:usize = 0;
        let slot_player = &find_slot_player![self.current_player, Slot::PlayerOne, Slot::PlayerTwo];
        let slot_enemy = &find_slot_enemy![self.current_player, Slot::PlayerOne, Slot::PlayerTwo];

        for dir in Direction::iterator()
        {
            count += self.is_capture(dir, (x, y), (slot_player, slot_enemy), with_delete);
        }
        count
    }

    fn is_capture(&mut self, dir: &Direction, (x, y):(i32, i32), (slot_player, slot_enemy): (&Slot, &Slot), with_delete: bool) -> usize
    {
        if dir.next_three((x, y), self) == (slot_enemy, slot_enemy, slot_player)
        {
            if with_delete
            {
                dir.capture((x, y), self);
                match slot_player
                {
                    &Slot::PlayerOne => self.players_score.0 += 2,
                    _                => self.players_score.1 += 2
                }
                println!("Score: {:?}", self.players_score);
                if self.players_score.0 >= 10 || self.players_score.1 >= 10
                {
                    self.is_finish = true;
                    println!("Finish");
                }
            }
            2
        }
        else
        {
            0
        }
    }

    fn is_double_three_move(&self, (x, y):(i32, i32)) -> Slot
    {
        match self.three_move_number((x, y), find_slot_player![self.current_player, Slot::PlayerOne, Slot::PlayerTwo])
        {
            2 => Slot::Forbidden,
            _ => Slot::Empty,
        }
    }

    fn three_move_number(&self, (x, y):(i32, i32), slot_player: Slot) -> usize
    {
        let mut count:usize = 0;
        let slot_enemy = find_slot_enemy![self.current_player, Slot::PlayerOne, Slot::PlayerTwo];

        for axe in Direction::axes_iterator()
        {
            if self.is_free_three((x, y), (&slot_player, &slot_enemy), axe)
            {
                count += 1;
                if count == 2
                {
                    return 2
                }
            }
        }
        count
    }

    fn is_free_three(&self, (x, y):(i32, i32), (slot_player, slot_enemy): (&Slot, &Slot), &(ref dir_add, ref dir_sub): &(Direction, Direction)) -> bool
    {
        let mut add = dir_add.new_coordonate((x, y));
        let mut sub = dir_sub.new_coordonate((x, y));

        let slot_add_one = self.find_value(add);
        let slot_sub_one = self.find_value(sub);

        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);
        
        let slot_add_two = self.find_value(add);
        let slot_sub_two = self.find_value(sub);

        if slot_cmp_or![&Slot::Forbidden; [slot_add_one, slot_sub_one]]
            || slot_cmp_or![slot_enemy; [slot_add_one, slot_sub_one]]
        {
            return false;
        }
        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);

        let slot_add_three = self.find_value(add);
        let slot_sub_three = self.find_value(sub);

        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);

        let slot_add_four = self.find_value(add);
        let slot_sub_four = self.find_value(sub);

        let total_add = slot_cmp![slot_player; (slot_add_one, slot_add_two, slot_add_three, slot_add_four)];
        let total_sub = slot_cmp![slot_player; (slot_sub_one, slot_sub_two, slot_sub_three, slot_sub_four)];
		
        total_add + total_sub == 2 && (total_add == 2 || total_sub == 2 || slot_cmp_or![slot_player; [slot_add_one, slot_sub_one]])
    }

    pub fn find_value(&self, (x, y):(i32, i32)) -> &Slot
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return &Slot::Forbidden;
        }
        &self.value[y as usize][x as usize]
    }

    pub fn is_winning_move(&self, (x, y):(i32, i32)) -> bool
    {
        for axe in Direction::axes_iterator()
        {
            if self.is_five_align((x, y), find_slots_players!(self.current_player), axe)
            {
                println!("FIVE ALIGN FOR PLAYER {:?} !!!", find_slot_player!(self.current_player, Slot::PlayerOne, Slot::PlayerTwo));
                return true;
            }
        }
        false
    }

    fn is_five_align(&self, (x, y):(i32, i32), (slot_player, _slot_enemy): (&Slot, &Slot), &(ref dir_add, ref dir_sub): &(Direction, Direction)) -> bool
    {
        let mut add = dir_add.new_coordonate((x, y));
        let mut sub = dir_sub.new_coordonate((x, y));

        let slot_add_one = self.find_value(add);
        let slot_sub_one = self.find_value(sub);

        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);
        
        let slot_add_two = self.find_value(add);
        let slot_sub_two = self.find_value(sub);

        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);

        let slot_add_three = self.find_value(add);
        let slot_sub_three = self.find_value(sub);

        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);

        let slot_add_four = self.find_value(add);
        let slot_sub_four = self.find_value(sub);

        let total_add = slot_winning![&slot_player; [slot_add_one, slot_add_two, slot_add_three, slot_add_four]];
        let total_sub = slot_winning![&slot_player; [slot_sub_one, slot_sub_two, slot_sub_three, slot_sub_four]];
		
        // println!("dir {:?} : =>  total add {} total sub {}", (dir_add, dir_sub), total_add, total_sub);
        total_add + total_sub >= 4 // 4 because the current slot isn't taking in consideration in slot_winning! macro
    }

    pub fn update_hint_map(&mut self, (x, y):(i32, i32), deaph: usize) -> ()
    {
        for dir in Direction::iterator()
        {
            let mut coord = dir.new_coordonate((x, y));
            if self.find_value(coord) == &Slot::Empty
                && self.number_captured(coord, false) > 0
            {
                self.hint_map[y as usize][x as usize] = find_slot_player![self.current_player, HintSlot::CapturePlayerOne, HintSlot::CapturePlayerTwo];
                continue ;
            }
            coord = dir.new_coordonate(coord);
            if self.find_value(coord) == &Slot::Empty
                && self.number_captured(coord, false) > 0
            {
                self.hint_map[y as usize][x as usize] = find_slot_player![self.current_player, HintSlot::CapturePlayerOne, HintSlot::CapturePlayerTwo];
                continue ;
            }
            if deaph < 3
            {
                continue ;
            }
            coord = dir.new_coordonate(coord);
            if self.find_value(coord) == &Slot::Empty
                && self.number_captured(coord, false) > 0
            {
                self.hint_map[y as usize][x as usize] = find_slot_player![self.current_player, HintSlot::CapturePlayerOne, HintSlot::CapturePlayerTwo];
            }
        }
    }
}
