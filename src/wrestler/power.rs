#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Power(i32);

impl Power {
    pub fn try_new(i: i32) -> Result<Power, String> {
        if (1..=100).contains(&i) {
            Ok(Self(i))
        } else {
            Err("Power must be 1 to 100".to_owned())
        }
    }

    pub fn diff(&self, opposing_power: &Power) -> i32 {
        self.0 - opposing_power.0
    }
}

#[cfg(test)]
mod tests {
    use crate::wrestler::Power;
    use claims::{assert_err, assert_gt, assert_lt, assert_ok};

    #[test]
    fn try_new_allows_valid_power_values() {
        assert_ok!(Power::try_new(1));
        assert_ok!(Power::try_new(100));
    }

    #[test]
    fn try_new_errors_on_invalid_power_values() {
        assert_err!(Power::try_new(0));
        assert_err!(Power::try_new(-1));
        assert_err!(Power::try_new(101));
    }

    #[test]
    fn power_can_be_compared() {
        let one_hundred_power = Power::try_new(100).unwrap();
        let ninety_nine_power = Power::try_new(99).unwrap();

        assert_gt!(one_hundred_power, ninety_nine_power);

        let one_power = Power::try_new(1).unwrap();
        let two_power = Power::try_new(2).unwrap();

        assert_lt!(one_power, two_power);

        let fifty_power_a = Power::try_new(50).unwrap();
        let fifty_power_b = Power::try_new(50).unwrap();

        assert_eq!(fifty_power_a, fifty_power_b);
    }

    #[test]
    fn diff_of_same_power_returns_0() {
        let fifty_power_a = Power::try_new(50).unwrap();
        let fifty_power_b = Power::try_new(50).unwrap();

        assert_eq!(0, fifty_power_a.diff(&fifty_power_b));
    }

    #[test]
    fn diff_of_lower_vs_higher_returns_negative() {
        let one_power = Power::try_new(1).unwrap();
        let one_hundred_power = Power::try_new(100).unwrap();

        assert_eq!(-99, one_power.diff(&one_hundred_power));
    }

    #[test]
    fn diff_of_higher_vs_lower_returns_positive() {
        let one_power = Power::try_new(1).unwrap();
        let one_hundred_power = Power::try_new(100).unwrap();

        assert_eq!(99, one_hundred_power.diff(&one_power));
    }
}
