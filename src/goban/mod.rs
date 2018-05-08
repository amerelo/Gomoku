#[macro_use]
pub mod macros;
pub mod player;
pub mod map;
pub mod finish;

#[cfg(test)]
mod tests
{
    use goban::player::{Player};
    use goban::map::{Map};
    use heuristic;

    fn init() -> Map
    {
        let mut map = Map {..Default::default() };

        map.set_value((3, 0), 1);
        map.set_value((0, 3), 1);
        map.set_value((3, 2), 1);
        map.set_value((5, 2), 1);
        map.set_value((3, 12), 1);
        map.set_value((4, 13), 1);
        map.set_value((3, 11), 1);
        map.set_value((3, 11), 1);
        map
    }

    fn init_with_free_three() -> Map
    {
        let mut map = Map {..Default::default() };

        map.set_value((1, 3), 1);
        map.set_value((1, 5), 1);
        map.set_value((3, 4), 1);
	    map.set_value((2, 4), 1);

        map
    }

    fn init_with_five_slot() -> Map
    {
        let mut map = Map {..Default::default() };

        map.set_value((5, 4), 1);
        map.set_value((3, 4), 1);
	    map.set_value((2, 4), 1);
        map.set_value((1, 4), 1);

        map
    }

	#[test]
	fn slot_is_available_0()
    {
		assert_eq!(init().is_available((0, 2)), 0);
	}

	#[test]
	fn slot_is_unavailable_1()
    {
        let map = init();

		assert_eq!(map.is_available((0, 4)), 0);
		assert_eq!(map.is_available((19, 4)), -1); // overflow
	}

	#[test]
	fn change_players_2()
    {
        let mut map = init();

		assert_eq!(map.current_player, Player::One);
        map.change_player_turn();
		assert_eq!(map.current_player, Player::Two);
	}

    #[test]
	fn two_free_three_move_3()
    {
        let mut map = init_with_free_three();

        // x == 1 && y == 4
		assert_eq!(map.is_available((1, 4)), -1);
	}

    // #[test]
	// fn winning_five_6()
    // {
    //     let mut map = init_with_five_slot();

	// 	assert_eq!(map.is_winning_move((4, 4)), true);
    //     map.set_value((4, 5), 2);
    //     map.set_value((4, 3), 3);
	// 	assert_eq!(map.is_winning_move((4, 4)), false);
	// 	assert_eq!(map.is_winning_move((5, 3)), false);
	// }
}
