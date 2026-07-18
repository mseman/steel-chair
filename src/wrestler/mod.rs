mod power;

pub use power::Power;

use uuid::Uuid;

pub const BASE_WIN_CHANCE: i32 = 50;
pub const MAX_WIN_CHANCE: i32 = 98;
pub const MIN_WIN_CHANCE: i32 = 2;

#[derive(Debug, Eq)]
pub struct Wrestler {
    id: Uuid,
    power: Power,
}

impl Wrestler {
    pub fn new(power: Power) -> Self {
        Self {
            id: Uuid::now_v7(),
            power,
        }
    }

    pub fn try_new(raw_power: i32) -> Result<Self, String> {
        let power = Power::try_new(raw_power)?;
        Ok(Wrestler::new(power))
    }

    pub fn get_win_chance(&self, opponent: &Wrestler) -> u32 {
        let chance = BASE_WIN_CHANCE + self.power.diff(&opponent.power);
        chance.clamp(MIN_WIN_CHANCE, MAX_WIN_CHANCE) as u32
    }
}

impl PartialEq for Wrestler {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::wrestler::{MAX_WIN_CHANCE, MIN_WIN_CHANCE, Wrestler};

    #[test]
    fn eq_returns_true_with_same_wrestler() {
        let wrestler_one = Wrestler::try_new(77).unwrap();

        assert_eq!(wrestler_one, wrestler_one);
    }

    #[test]
    fn eq_returns_false_for_wrestlers_with_different_power() {
        let wrestler_one = Wrestler::try_new(99).unwrap();
        let wrestler_two = Wrestler::try_new(12).unwrap();

        assert_ne!(wrestler_one, wrestler_two);
    }

    #[test]
    fn eq_returns_false_for_wrestlers_with_same_power() {
        let wrestler_one = Wrestler::try_new(77).unwrap();
        let wrestler_two = Wrestler::try_new(77).unwrap();

        assert_ne!(wrestler_one, wrestler_two);
    }

    #[test]
    fn get_win_chance_returns_50_with_equal_power() {
        let wrestler_one = Wrestler::try_new(77).unwrap();
        let wrestler_two = Wrestler::try_new(77).unwrap();
        assert_eq!(50, wrestler_one.get_win_chance(&wrestler_two));
    }

    #[test]
    fn get_win_chance_returns_greater_than_50_when_higher_power() {
        let wrestler_one = Wrestler::try_new(90).unwrap();
        let wrestler_two = Wrestler::try_new(50).unwrap();
        assert_eq!(90, wrestler_one.get_win_chance(&wrestler_two));
    }

    #[test]
    fn get_win_chance_returns_less_than_50_when_lower_power() {
        let wrestler_one = Wrestler::try_new(40).unwrap();
        let wrestler_two = Wrestler::try_new(60).unwrap();
        assert_eq!(30, wrestler_one.get_win_chance(&wrestler_two));
    }

    #[test]
    fn get_win_chance_cannot_be_greater_than_max() {
        let wrestler_one = Wrestler::try_new(100).unwrap();
        let wrestler_two = Wrestler::try_new(1).unwrap();
        assert_eq!(
            MAX_WIN_CHANCE as u32,
            wrestler_one.get_win_chance(&wrestler_two)
        );
    }

    #[test]
    fn get_win_chance_cannot_be_lesser_than_min() {
        let wrestler_one = Wrestler::try_new(1).unwrap();
        let wrestler_two = Wrestler::try_new(100).unwrap();
        assert_eq!(
            MIN_WIN_CHANCE as u32,
            wrestler_one.get_win_chance(&wrestler_two)
        );
    }
}
