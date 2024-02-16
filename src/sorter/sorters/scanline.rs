use image::{DynamicImage, Rgba};
use crate::sorter::sorters::Sorter;
use crate::sorter::SortMethod;

pub struct ScanlineSorter;

impl Sorter<Rgba<u8>, &DynamicImage, (), ()> for ScanlineSorter {
    fn sort_image(&self, image: &DynamicImage, sorter: Box<dyn SortMethod<Rgba<u8>, ()>>) -> () {
        let rgba_image = image.to_rgba8();
        rgba_image.rows().enumerate().for_each(|(i, row)| {
            // println!("{i}");
            let pixels = row.copied().collect::<Vec<_>>();
            sorter.sort(pixels, i);
        });
    }
}

















