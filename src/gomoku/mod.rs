pub mod player;
pub mod direction;
pub mod map;

#[cfg(test)]
mod tests
{
    use gomoku::player::{Player, PlayerKind};
    use gomoku::direction::{Direction};
    use gomoku::map::{Map, Slot};

    fn init() -> Map
    {
        let mut test = Map {..Default::default() };

        test.value[0][3] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[3][0] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[2][3] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[12][3] = Slot::Used(Player::Two(PlayerKind::Human));
        test.value[13][4] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[2][5] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[11][3] = Slot::Used(Player::One(PlayerKind::AI));
        test.value[11][3] = Slot::Used(Player::One(PlayerKind::AI));
        test
    }

	#[test]
	fn slot_is_available_0()
    {
		assert_eq!(init().is_available((0, 2), Player::One(PlayerKind::AI)), true);
	}

	#[test]
	fn slot_is_unavailable_1()
    {
        let test = init();

		assert_eq!(test.is_available((0, 3), Player::One(PlayerKind::AI)), false);
		assert_eq!(test.is_available((19, 4), Player::One(PlayerKind::AI)), false); // overflow
	}

	#[test]
	fn move_is_authorize_2()
    {
        let test = init();

		assert_eq!(test.move_authorize(0, 3, Direction::Down, Player::One(PlayerKind::AI)), true);
		assert_eq!(test.move_authorize(0, 3, Direction::Right, Player::One(PlayerKind::AI)), true);
	}

	#[test]
	fn move_is_forbidden_3()
    {
        let test = init();

		assert_eq!(test.move_authorize(0, 0, Direction::Up, Player::One(PlayerKind::AI)), false);
		assert_eq!(test.move_authorize(0, 0, Direction::UpLeft, Player::One(PlayerKind::AI)), false);
		assert_eq!(test.move_authorize(0, 3, Direction::Left, Player::One(PlayerKind::AI)), false);
	}
}
