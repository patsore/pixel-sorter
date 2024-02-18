use egui::{Color32, ColorImage};

use rayon::prelude::*;
use crate::sorter::sorters::Sorter;
use crate::sorter::SortMethod;

pub struct ScanlineSorter;

impl Sorter<Color32, &mut ColorImage, (), ()> for ScanlineSorter {
    fn sort_image(&self, image: &mut ColorImage, sorter: impl SortMethod<Color32, ()>) -> () {
        let pixels: &mut Vec<Color32> = image.pixels.as_mut();
        let [w, _] = image.size;
        pixels.par_chunks_exact_mut(w).for_each(|row| {
            sorter.sort(row);
        });
    }
}

















