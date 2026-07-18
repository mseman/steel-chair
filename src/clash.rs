use rand::Rng;

use crate::wrestler::Wrestler;

pub fn get_winner<'a>(
    wrestler_one: &'a Wrestler,
    wrestler_two: &'a Wrestler,
    rng: &mut impl Rng,
) -> &'a Wrestler {
    let wrestler_one_win_chance = wrestler_one.get_win_chance(wrestler_two);
    // TODO: change to rng.random_range(1..=100) once FakeRng impls RngExt
    let dice_roll = (rng.next_u32() % 100) + 1;

    if dice_roll <= wrestler_one_win_chance {
        wrestler_one
    } else {
        wrestler_two
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::clash::get_winner;
    use crate::test_helpers::FakeRng;
    use crate::wrestler::{MAX_WIN_CHANCE, MIN_WIN_CHANCE, Wrestler};

    fn test_get_winner_sequence(
        range: Range<u32>,
        wrestler_one: &Wrestler,
        wrestler_two: &Wrestler,
        expected_winner: &Wrestler,
    ) {
        for i in range {
            let mut rng = FakeRng::new(vec![i]);
            assert_eq!(
                expected_winner,
                get_winner(wrestler_one, wrestler_two, &mut rng)
            );
        }
    }

    #[test]
    fn get_winner_returns_correct_winner_when_equal_power() {
        let wrestler_one = Wrestler::try_new(99).unwrap();
        let wrestler_two = Wrestler::try_new(99).unwrap();

        test_get_winner_sequence(0..50, &wrestler_one, &wrestler_two, &wrestler_one);

        test_get_winner_sequence(50..100, &wrestler_one, &wrestler_two, &wrestler_two);
    }

    #[test]
    fn get_winner_returns_correct_winner_when_greater_power() {
        let wrestler_90 = Wrestler::try_new(90).unwrap();
        let wrestler_70 = Wrestler::try_new(70).unwrap();

        test_get_winner_sequence(0..70, &wrestler_90, &wrestler_70, &wrestler_90);

        test_get_winner_sequence(70..100, &wrestler_90, &wrestler_70, &wrestler_70);
    }

    #[test]
    fn get_winner_returns_correct_winner_when_lesser_power() {
        let wrestler_90 = Wrestler::try_new(90).unwrap();
        let wrestler_70 = Wrestler::try_new(70).unwrap();

        test_get_winner_sequence(0..30, &wrestler_70, &wrestler_90, &wrestler_70);

        test_get_winner_sequence(30..100, &wrestler_70, &wrestler_90, &wrestler_90);
    }

    #[test]
    fn get_winner_returns_correct_winner_at_edges_of_power_diff() {
        let wrestler_100 = Wrestler::try_new(100).unwrap();
        let wrestler_1 = Wrestler::try_new(1).unwrap();

        let range = 0..(MAX_WIN_CHANCE as u32);
        test_get_winner_sequence(range, &wrestler_100, &wrestler_1, &wrestler_100);

        let range = (MAX_WIN_CHANCE as u32)..100;
        test_get_winner_sequence(range, &wrestler_100, &wrestler_1, &wrestler_1);

        let range = 0..(MIN_WIN_CHANCE as u32);
        test_get_winner_sequence(range, &wrestler_1, &wrestler_100, &wrestler_1);

        let range = (MIN_WIN_CHANCE as u32)..100;
        test_get_winner_sequence(range, &wrestler_1, &wrestler_100, &wrestler_100);
    }
}
