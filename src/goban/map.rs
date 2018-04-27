use goban::player::{Player, PlayerKind};
use goban::direction::{Direction};
use std::ops::Add;

const  SIZEMAP: usize = 19;

#[derive(Debug, PartialEq)]
pub enum Slot
{
    PlayerOne,
    PlayerTwo,
    Empty,
    Forbidden,
}

#[derive(Debug)]
pub struct Map
{
    pub value: Vec<Vec<Slot>>,
    pub players_kind: (PlayerKind, PlayerKind), // easier way for handling players number and players kind
    pub current_player: Player,
}

impl Default for Map
{
    fn default() -> Map
    {
        Map
        {
            value: mapinit![SIZEMAP],
            players_kind: (PlayerKind::Human, PlayerKind::Human),
            current_player: Player::One,
        }
    }
}

impl Map
{
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

    fn is_double_three_move(&self, (x, y):(i32, i32)) -> Slot
    {
        match self.three_move_number((x, y), find_slot_player![self.current_player])
        {
            2 => Slot::Forbidden,
            _ => Slot::Empty,
        }
    }

// FAUT TOUT PROPRIFIER C'EST TROP SALE

    fn three_move_number(&self, (x, y):(i32, i32), slot_player: Slot) -> usize
    {
        let mut count:usize = 0;
        let slot_enemy = match slot_player
        {
            Slot::PlayerOne => Slot::PlayerTwo,
            _               => Slot::PlayerOne
        };

        if self.is_free_three((x, y), (&slot_player, &slot_enemy), (Direction::Up, Direction::Down))
        {
            count += 1;
        }
        if self.is_free_three((x, y), (&slot_player, &slot_enemy), (Direction::Left, Direction::Right))
        {
            count += 1;
        }
        if count == 2
        {
            return 2
        }
        if self.is_free_three((x, y), (&slot_player, &slot_enemy), (Direction::UpLeft, Direction::DownRight))
        {
            count += 1;
        }
        if count == 2
        {
            return 2
        }
        if self.is_free_three((x, y), (&slot_player, &slot_enemy), (Direction::UpRight, Direction::DownLeft))
        {
            count += 1;
        }
        if count == 2
        {
            2
        }
        else
        {
            0
        }
    }

    fn is_free_three(&self, (x, y):(i32, i32), (slot_player, slot_enemy): (&Slot, &Slot), (dir_add, dir_sub): (Direction, Direction)) -> bool
    {
        let mut count:usize = 0;

        // println!("values {:?}", self.value[1][3]);
        let mut add = dir_add.new_coordonate((x, y));
        let mut sub = dir_sub.new_coordonate((x, y));
        // println!("add {:?}{:?} sub {:?}{:?}", add, self.find_value(add), sub, self.find_value(sub));

        let slot_add_one = self.find_value(add);
        let slot_sub_one = self.find_value(sub);

        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);
        // println!("add {:?}{:?} sub {:?}{:?}", add, self.find_value(add), sub, self.find_value(sub));
        
        let slot_add_two = self.find_value(add);
        let slot_sub_two = self.find_value(sub);

        if slot_cmp_or![&Slot::Forbidden; [slot_add_one, slot_sub_one]]
            || slot_cmp_or![slot_enemy; [slot_add_one, slot_sub_one]]
        {
            return false;
        }
        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);
        // println!("add {:?}{:?} sub {:?}{:?}", add, self.find_value(add), sub, self.find_value(sub));

        let slot_add_three = self.find_value(add);
        let slot_sub_three = self.find_value(sub);

        add = dir_add.new_coordonate(add);
        sub = dir_sub.new_coordonate(sub);
        // println!("add {:?}{:?} sub {:?}{:?}", add, self.find_value(add), sub, self.find_value(sub));

        let slot_add_four = self.find_value(add);
        let slot_sub_four = self.find_value(sub);

        let total_add = slot_cmp![slot_player; (slot_add_one, slot_add_two, slot_add_three, slot_add_four)];
        let total_sub = slot_cmp![slot_player; (slot_sub_one, slot_sub_two, slot_sub_three, slot_sub_four)];
		
        // println!("add {:?}{:?} sub {:?}{:?}", add, self.find_value(add), sub, self.find_value(sub));
        // println!("total_add {:?} total_sub {:?}", total_add, total_sub);
        total_add + total_sub == 2 && (total_add == 2 || total_sub == 2 || slot_cmp_or![slot_player; [slot_add_one, slot_sub_one]])
    }

    fn find_value(&self, (x, y):(i32, i32)) -> &Slot
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return &Slot::Forbidden;
        }
        &self.value[y as usize][x as usize]
    }    
}
