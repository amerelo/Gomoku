pub mod player;
pub mod direction;
pub mod map;

#[cfg(test)]
mod tests
{
    use goban::player::{Player, PlayerKind};
    use goban::direction::{Direction};
    use goban::map::{Map, Slot};

    fn init() -> Map
    {
        let mut test = Map {..Default::default() };

        test.value[0][3] = Slot::PlayerOne;
        test.value[3][0] = Slot::PlayerOne;
        test.value[2][3] = Slot::PlayerOne;
        test.value[12][3] = Slot::PlayerOne;
        test.value[13][4] = Slot::PlayerOne;
        test.value[2][5] = Slot::PlayerOne;
        test.value[11][3] = Slot::PlayerOne;
        test.value[11][3] = Slot::PlayerOne;
        test
    }

	#[test]
	fn slot_is_available_0()
    {
		assert_eq!(init().is_available((0, 2)), Slot::Empty);
	}

	#[test]
	fn slot_is_unavailable_1()
    {
        let test = init();

		assert_eq!(test.is_available((0, 3)), Slot::PlayerOne);
		assert_eq!(test.is_available((19, 4)), Slot::Forbidden); // overflow
	}

	#[test]
	fn move_is_authorize_2()
    {
        let test = init();

		assert_eq!(test.move_authorize(0, 3, Direction::Down), true);
		assert_eq!(test.move_authorize(0, 3, Direction::Right), true);
	}

	#[test]
	fn move_is_forbidden_3()
    {
        let test = init();

		assert_eq!(test.move_authorize(0, 0, Direction::Up), false);
		assert_eq!(test.move_authorize(0, 0, Direction::UpLeft), false);
		assert_eq!(test.move_authorize(0, 3, Direction::Left), false);
	}

	#[test]
	fn change_players_4()
    {
        let mut test = init();

		assert_eq!(test.current_player, Player::One);
        test.change_player_turn();
		assert_eq!(test.current_player, Player::Two);
	}
}
