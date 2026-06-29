use rand::{Rng, RngExt};
use std::cmp::Ordering;

use crate::wrestler::Wrestler;

pub fn get_winner<'a>(
    wrestler_one: &'a Wrestler,
    wrestler_two: &'a Wrestler,
    rng: &mut impl Rng,
) -> &'a Wrestler {
    match wrestler_one.power.cmp(&wrestler_two.power) {
        Ordering::Greater => wrestler_one,
        Ordering::Less => wrestler_two,
        Ordering::Equal => {
            if rng.random_bool(0.5) {
                wrestler_one
            } else {
                wrestler_two
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::Xoshiro128PlusPlus};

    use crate::clash::get_winner;
    use crate::wrestler::Wrestler;

    #[test]
    fn returns_wrestler_with_higher_power() {
        let wrestler_one = Wrestler::try_new(2).unwrap();
        let wrestler_two = Wrestler::try_new(1).unwrap();

        assert_eq!(
            &wrestler_one,
            get_winner(
                &wrestler_one,
                &wrestler_two,
                &mut Xoshiro128PlusPlus::seed_from_u64(1)
            )
        );
        assert_eq!(
            &wrestler_one,
            get_winner(
                &wrestler_two,
                &wrestler_one,
                &mut Xoshiro128PlusPlus::seed_from_u64(1)
            )
        );
    }

    #[test]
    fn returns_random_winner_when_equal_power() {
        let wrestler_one = Wrestler::try_new(99).unwrap();
        let wrestler_two = Wrestler::try_new(99).unwrap();

        const WRESTLER_ONE_WINNER_SEED: u64 = 72310981;
        assert_eq!(
            &wrestler_one,
            get_winner(
                &wrestler_one,
                &wrestler_two,
                &mut Xoshiro128PlusPlus::seed_from_u64(WRESTLER_ONE_WINNER_SEED)
            ),
            "wrestler_one failed to randomly win"
        );

        const WRESTLER_TWO_WINNER_SEED: u64 = 1832534;
        assert_eq!(
            &wrestler_two,
            get_winner(
                &wrestler_one,
                &wrestler_two,
                &mut Xoshiro128PlusPlus::seed_from_u64(WRESTLER_TWO_WINNER_SEED)
            ),
            "wrestler_two failed to randomly win"
        );
    }
}
