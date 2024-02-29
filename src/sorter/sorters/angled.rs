use crate::sorter::animation::Animateable;
use crate::sorter::sorters::Sorter;
use crate::sorter::SortMethod;
use egui::{Color32, ColorImage, Slider, Ui};
use rayon::prelude::*;
#[derive(Clone)]
pub struct AngledSorter {
    pub angle: f32,
}

//This code is absolutely horrible and causes undefined behavior
//but it makes things go fast and it doesn't crash immediately so until I
//decide to make a fix it's staying like this
impl Sorter<Color32, &mut ColorImage, (), ()> for AngledSorter {
    fn sort_image(&self, image: &mut ColorImage, sorter: impl SortMethod<Color32, ()>) -> () {
        let pixels: &mut Vec<Color32> = image.pixels.as_mut();
        let pixels_pointer = unsafe {
            let temp = std::ptr::read(pixels);
            let ptr = temp.into_raw_parts().0;
            ptr as usize
        };
        let [w, h] = image.size;

        if self.angle % 90.0 == 0.0 && self.angle != 0.0 {
            return;
        }

        let angle_tan = self.angle.to_radians().tan();

        let extra_height = (angle_tan * w as f32).floor() as i64;
        let range = if extra_height > 0 {
            -extra_height..h as i64
        } else {
            0..(h as i64 - extra_height)
        };

        range.par_bridge().for_each(|row| {
            let idxes = (0..w)
                .map(|xv| (xv, (xv as f32 * angle_tan + row as f32) as usize))
                .filter(|(_, y)| *y > 0 && *y < h)
                .collect::<Vec<_>>();

            let mut angled_pixels = idxes
                .iter()
                .map(|(x, y)| unsafe { *pixels.get_unchecked(y * w + x) })
                .collect::<Vec<_>>();
            sorter.sort(&mut angled_pixels[..]);

            for (i, (x, y)) in idxes.iter().enumerate() {
                unsafe {
                    std::ptr::write(
                        (pixels_pointer + (y * w + x) * std::mem::size_of::<Color32>())
                            as *mut Color32,
                        *angled_pixels.get_unchecked(i),
                    );
                }
            }
        });
    }

    fn ui(&mut self, ui: &mut Ui) {
        let angle = Slider::new(&mut self.angle, 0.0..=360.0)
            .text("Angle to sort at")
            .drag_value_speed(0.1);
        ui.add(angle);
    }
}

impl Animateable for AngledSorter {
    fn lerp(&mut self, target: &AngledSorter, weight: f32) {
        self.angle += ( target.angle - self.angle) * weight;
    }
}
