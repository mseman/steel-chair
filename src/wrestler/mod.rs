mod power;

pub use power::Power;

use uuid::Uuid;

#[derive(Debug, Eq)]
pub struct Wrestler {
    id: Uuid,
    pub power: Power,
}

impl Wrestler {
    pub fn new(power: Power) -> Self {
        Self {
            id: Uuid::now_v7(),
            power,
        }
    }

    pub fn try_new(raw_power: i32) -> Result<Self, String> {
        let power = Power::parse(raw_power)?;
        Ok(Wrestler::new(power))
    }
}

impl PartialEq for Wrestler {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
