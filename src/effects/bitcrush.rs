use crate::effects::traits::AudioEffect;

pub struct Bitcrush {
    bit_depth: f32 ,
    sample_hold : usize
}

impl Bitcrush {
    pub fn new(bit_depth: usize, sample_hold: usize) -> Self {
        assert!(bit_depth > 0 && bit_depth <= 32);
        assert!(sample_hold > 0);
        let bit_depth = bit_depth as f32;
        Bitcrush {
            bit_depth,
            sample_hold
        }
    }
}

impl AudioEffect for Bitcrush{
    fn process_sample(&self, input: f32) -> f32 {
        let max :f32 = 2f32.powf(self.bit_depth);
        ((input + 1.0).round() * max) / max - 1.0
    }

    fn process_block<'a>(&self, input: &'a [f32]) -> &'a[f32] {
        let mut output = input;
        output
    }
}
