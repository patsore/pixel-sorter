pub trait Animateable {
    fn lerp(&mut self, target: &Self, weight: f32);
}
