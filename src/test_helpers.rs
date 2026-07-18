use rand::rand_core::{Infallible, TryRng, utils};

#[derive(Debug, Clone)]
pub struct FakeRng {
    sequence: Vec<u32>,
    index: usize,
}

impl FakeRng {
    pub fn new(sequence: Vec<u32>) -> Self {
        debug_assert!(
            !sequence.is_empty(),
            "FakeRng requires a non-empty sequence"
        );
        Self { sequence, index: 0 }
    }
}

impl TryRng for FakeRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let val = self.sequence[self.index];
        self.index = (self.index + 1) % self.sequence.len();
        Ok(val)
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        utils::next_u64_via_u32(self)
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        utils::fill_bytes_via_next_word(dst, || self.try_next_u32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn fake_rng_loops_through_a_sequence_with_next_32() {
        let mut rng = FakeRng::new(vec![1, 2, 3]);
        let result = (0..7).map(|_| rng.next_u32()).collect::<Vec<u32>>();

        assert_eq!(result, vec![1, 2, 3, 1, 2, 3, 1]);
    }
}
