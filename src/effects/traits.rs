pub trait AudioEffect {
    fn process_sample(&self, input: f32) -> f32;
    fn process_block<'a>(&self, input: &'a [f32]) -> &'a [f32];
}