use std::cell::Cell;
use egui::{Color32, ColorImage, Slider, Ui};
use rayon::prelude::*;
use crate::sorter::sorters::Sorter;
use crate::sorter::SortMethod;

#[derive(Clone)]
pub struct AngledSorter {
    pub angle: f32,
}

impl Sorter<Color32, &mut ColorImage, (), ()> for AngledSorter {
    fn sort_image(&self, image: &mut ColorImage, sorter: impl SortMethod<Color32, ()>) -> () {
        let pixels: &mut Vec<Color32> = image.pixels.as_mut();
        let [w, h] = image.size;

        let angle_tan = self.angle.to_radians().tan();
        let extra_height = (angle_tan * w as f32).floor() as i64;
        let range = if extra_height > 0 {
            -extra_height..h as i64
        } else {
            0..(h as i64 - extra_height)
        };


        let px_len = pixels.len();
        let mut result_pixels_temp = range.par_bridge().map(|row| {
            let mut idxes = (0..w)
                .par_bridge()
                .map(|xv| (xv, (xv as f32 * angle_tan + row as f32) as usize))
                .filter(|(_, y)| *y > 0 && *y < h)
                .collect::<Vec<_>>();

            unsafe {
                let mut pixels = idxes
                    .par_iter()
                    .map(|&(x, y)| pixels.get_unchecked(y * w + x).clone())
                    .collect::<Vec<_>>();
                sorter.sort(&mut pixels[..]);
                let index_px = idxes.drain(..).zip(pixels.drain(..)).collect::<Vec<_>>();
                return index_px;
            }
        }).flatten().collect::<Vec<_>>();

        for ((x, y), px) in result_pixels_temp {
            pixels[y * w + x] = px;
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        let angle = Slider::new(&mut self.angle, 0.0..=360.0).text("Angle to sort at").drag_value_speed(0.1);
        ui.add(angle);
    }
}

















