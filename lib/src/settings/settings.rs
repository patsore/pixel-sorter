use crate::PixelCharacteristic;

#[derive(Clone, Copy)]
pub struct Settings {
    pub sort_path_angle: f32,
    pub sort_path: SortingPath,
    pub threshold: Threshold,
    pub sort_by: PixelCharacteristic,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            sort_path_angle: 45.0,
            sort_path: SortingPath::Linear,
            threshold: Threshold::default(),
            sort_by: PixelCharacteristic::Average,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Threshold {
    pub min: u8,
    pub max: u8,
    pub threshold_type: PixelCharacteristic,
    // pub override_transparency_failsafe: bool,
}

impl Default for Threshold {
    fn default() -> Self {
        Threshold {
            min: 0,
            max: 255,
            threshold_type: PixelCharacteristic::Average,
            // override_transparency_failsafe: false,
        }
    }
}

impl Threshold {
    pub fn contains(&self, to_check: u8) -> bool {
        (self.min <= to_check) && (to_check <= self.max)
    }
}

#[derive(Clone, Copy)]
pub enum SortingPath {
    Linear,
}
