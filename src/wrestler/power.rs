#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Power(i32);

impl Power {
    pub fn parse(i: i32) -> Result<Power, String> {
        if (1..=100).contains(&i) {
            Ok(Self(i))
        } else {
            Err("Power must be 1 to 100".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wrestler::Power;
    use claims::{assert_err, assert_gt, assert_lt, assert_ok};

    #[test]
    fn one_is_a_valid_power() {
        assert_ok!(Power::parse(1));
    }

    #[test]
    fn one_hundred_is_a_valid_power() {
        assert_ok!(Power::parse(100));
    }

    #[test]
    fn zero_is_not_a_valid_power() {
        assert_err!(Power::parse(0));
    }

    #[test]
    fn negative_one_is_not_a_valid_power() {
        assert_err!(Power::parse(-1));
    }

    #[test]
    fn one_hundred_and_one_is_not_a_valid_power() {
        assert_err!(Power::parse(101));
    }

    #[test]
    fn one_hundred_power_greater_than_ninety_nine() {
        let one_hundred_power = Power::parse(100).unwrap();
        let ninety_nine_power = Power::parse(99).unwrap();

        assert_gt!(one_hundred_power, ninety_nine_power);
    }

    #[test]
    fn one_power_less_than_two() {
        let one_power = Power::parse(1).unwrap();
        let two_power = Power::parse(2).unwrap();

        assert_lt!(one_power, two_power);
    }

    #[test]
    fn fifty_power_equals_fifty_power() {
        let fifty_power_a = Power::parse(50).unwrap();
        let fifty_power_b = Power::parse(50).unwrap();

        assert_eq!(fifty_power_a, fifty_power_b);
    }
}
