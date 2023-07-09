use image::Rgba;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PixelCharacteristic {
    Hue,
    Saturation,
    Luminance,
    Average,
}

impl PixelCharacteristic {
    pub fn get_pixel_characteristic_value(self, pixel: &Rgba<u8>) -> u8 {
        match self {
            PixelCharacteristic::Hue => 0,
            PixelCharacteristic::Saturation => 0,
            PixelCharacteristic::Luminance => {
                (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32)
                    as u8
            }
            PixelCharacteristic::Average => {
                ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8
            }
        }
    }

    pub fn compare(&self, a: &Rgba<u8>, b: &Rgba<u8>) -> std::cmp::Ordering {
        self.get_pixel_characteristic_value(a)
            .partial_cmp(&self.get_pixel_characteristic_value(b))
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}
