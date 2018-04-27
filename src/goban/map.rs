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
        // if self.is_free_three((x, y), &slot_player, (Direction::UpLeft, Direction::DownRight))
        // {
        //     count += 1;
        // }
        // if self.is_free_three((x, y), &slot_player, (Direction::UpRight, Direction::DownLeft))
        // {
        //     count += 1;
        // }
        // if self.is_free_three((x, y), &slot_player, (Direction::Left, Direction::Right))
        // {
        //     count += 1;
        // }
        count
    }

    fn is_free_three(&self, (x, y):(i32, i32), (slot_player, slot_enemy): (&Slot, &Slot), (dir_add, dir_sub): (Direction, Direction)) -> bool
    {
        let mut count:usize = 0;

        let slot_add_one = self.value(dir_add.new_coordonate((x, y)));
        let slot_sub_one = self.value(dir_sub.new_coordonate((x, y)));
        
        let slot_add_two = self.value(dir_add.new_coordonate(dir_add.new_coordonate((x, y))));
        let slot_sub_two = self.value(dir_sub.new_coordonate(dir_sub.new_coordonate((x, y))));

        
        if slot_cmp![&Slot::Forbidden; [slot_add_one, slot_add_two, slot_sub_one, slot_sub_two]]
            || slot_cmp![slot_enemy; [slot_add_one, slot_add_two, slot_sub_one, slot_sub_two]]
        {
            return false;
        }

        let slot_add_three = self.value(dir_add.new_coordonate(dir_add.new_coordonate(dir_add.new_coordonate((x, y)))));
        let slot_sub_three = self.value(dir_sub.new_coordonate(dir_sub.new_coordonate(dir_sub.new_coordonate((x, y)))));

        true
    }

    fn value(&self, (x, y):(i32, i32)) -> &Slot
    {
        if x > 18 || y > 18 || x < 0 || y < 0
        {
            return &Slot::Forbidden;
        }
        &self.value[y as usize][x as usize]
    }    
}
