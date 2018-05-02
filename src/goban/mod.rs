#[macro_use]
pub mod macros;
pub mod player;
pub mod direction;
pub mod map;

#[cfg(test)]
mod tests
{
    use goban::player::{Player};
    use goban::direction::{Direction};
    use goban::map::{Map, slot::Slot};
    use heuristic;

    fn init() -> Map
    {
        let mut map = Map {..Default::default() };

        map.value[0][3] = Slot::PlayerOne;
        map.value[3][0] = Slot::PlayerOne;
        map.value[2][3] = Slot::PlayerOne;
        map.value[12][3] = Slot::PlayerOne;
        map.value[13][4] = Slot::PlayerOne;
        map.value[2][5] = Slot::PlayerOne;
        map.value[11][3] = Slot::PlayerOne;
        map.value[11][3] = Slot::PlayerOne;
        map
    }

    fn init_with_free_three() -> Map
    {
        let mut map = Map {..Default::default() };

        map.value[3][1] = Slot::PlayerOne;
        map.value[5][1] = Slot::PlayerOne;
        map.value[4][3] = Slot::PlayerOne;
	    map.value[4][2] = Slot::PlayerOne;

        map
    }

    fn init_with_five_slot() -> Map
    {
        let mut map = Map {..Default::default() };

        map.value[4][5] = Slot::PlayerOne;
        map.value[4][3] = Slot::PlayerOne;
	    map.value[4][2] = Slot::PlayerOne;
        map.value[4][1] = Slot::PlayerOne;

        map
    }

	#[test]
	fn slot_is_available_0()
    {
		assert_eq!(init().is_available((0, 2)), Slot::Empty);
	}

	#[test]
	fn slot_is_unavailable_1()
    {
        let map = init();

		assert_eq!(map.is_available((0, 3)), Slot::PlayerOne);
		assert_eq!(map.is_available((19, 4)), Slot::Forbidden); // overflow
	}

	#[test]
	fn move_is_authorize_2()
    {
        let map = init();

		assert_eq!(map.move_authorize(0, 3, Direction::Down), true);
		assert_eq!(map.move_authorize(0, 3, Direction::Right), true);
	}

	#[test]
	fn move_is_forbidden_3()
    {
        let map = init();

		assert_eq!(map.move_authorize(0, 0, Direction::Up), false);
		assert_eq!(map.move_authorize(0, 0, Direction::UpLeft), false);
		assert_eq!(map.move_authorize(0, 3, Direction::Left), false);
	}

	#[test]
	fn change_players_4()
    {
        let mut map = init();

		assert_eq!(map.current_player, Player::One);
        map.change_player_turn();
		assert_eq!(map.current_player, Player::Two);
	}

    #[test]
	fn two_free_three_move_5()
    {
        let mut map = init_with_free_three();

        // x == 1 && y == 4
		assert_eq!(map.is_available((1, 4)), Slot::Forbidden);
	}

    #[test]
	fn winning_five_6()
    {
        let mut map = init_with_five_slot();

		assert_eq!(map.is_winning_move((4, 4)), true);
        map.value[5][4] = Slot::PlayerTwo;
        map.value[3][4] = Slot::PlayerOne;
		assert_eq!(map.is_winning_move((4, 4)), false);
		assert_eq!(map.is_winning_move((5, 3)), false);
	}
}
