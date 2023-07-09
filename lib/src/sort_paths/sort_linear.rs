use crate::{sort_pixels_in_line, sort_pixels_in_line_new, Sorter};
use std::sync::Mutex;
use image::{GenericImage, GenericImageView};
use rayon::prelude::*;

impl Sorter {
    pub fn sort_linear(&mut self) {
        let rgba_image = self.current_image.as_mut_rgba8().unwrap();
        if self.settings.sort_path_angle != 0.0 && !(self.settings.sort_path_angle % 90.0 == 0.0) {
            let (w, h) = rgba_image.dimensions();

            let rgba_c = self.current_image.clone();

            let mutex_img = Mutex::from(&mut self.current_image);

            let tan = self.settings.sort_path_angle.to_radians().tan();
            let extra_height = (tan * w as f32).floor() as i64;
            let range = if extra_height > 0 {
                -extra_height..i64::from(h)
            } else {
                0..(i64::from(h) - extra_height)
            };

            range.par_bridge().for_each(|row| {
                let idxes = (0..w)
                    .map(|xv| (xv, (xv as f32 * tan + row as f32) as u32))
                    .filter(|(_, y)| *y > 0 && *y < h)
                    .collect::<Vec<_>>();

                let mut pixels = idxes
                    .iter()
                    .map(|(x, y)| rgba_c.get_pixel(*x, *y))
                    .collect::<Vec<_>>();

                sort_pixels_in_line_new(&self.settings, &mut pixels);

                let mut locked_image = mutex_img
                    .lock()
                    .expect("Couldn't acquire image lock");

                for ((idx_x, idx_y), px) in idxes.iter().zip(pixels.iter()) {
                    // mutex_img.lock().expect("Couldn't acquire image lock")
                    locked_image.put_pixel(*idx_x, *idx_y, *px);
                }
            })
        } else {
            let lines = rgba_image.rows_mut();
            lines.par_bridge().for_each(|line| {
                sort_pixels_in_line(&self.settings, &mut line.collect());
            });
        }
    }
}
