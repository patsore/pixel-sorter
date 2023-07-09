use std::mem;
use crate::{Settings, SortingPath};
use image::{DynamicImage, Rgba};

pub struct Sorter {
    pub original_image: DynamicImage,
    pub current_image: DynamicImage,
    pub settings: Settings,
}

impl Sorter {
    pub fn sort(&mut self) {
        match self.settings.sort_path {
            SortingPath::Linear => {
                self.sort_linear();
            }
        };
    }

    pub fn open_image(&mut self, path: &str) {
        self.original_image = DynamicImage::from((image::open(path).expect("Couldn't open image")).into_rgba8());
        self.current_image = self.original_image.clone();
    }

    pub fn reset_current_image(&mut self) {
        self.current_image = self.original_image.clone();
    }
}

pub fn sort_pixels_in_line(settings: &Settings, line: &mut Vec<&mut Rgba<u8>>) {
    let mut current_span: Vec<usize> = Vec::new();
    let mut spans: Vec<Vec<usize>> = Vec::new();

    for pixel in 0..line.len() {
        if settings.threshold.contains(
            settings
                .threshold
                .threshold_type
                .get_pixel_characteristic_value(&*line[pixel]),
        ) {
            current_span.push(pixel);
        } else if !current_span.is_empty() {
            spans.push(std::mem::take(&mut current_span));
        }
    }

    if !current_span.is_empty() {
        spans.push(mem::take(&mut current_span));
    }

    let temp_line: Vec<Rgba<u8>> = line.iter().map(|pixel| **pixel.clone()).collect();
    for span in &mut spans {
        let original_span = span.clone();
        span.sort_unstable_by(|a, b| settings.sort_by.compare(&*line[*a], &*line[*b]));

        for i in 0..span.len() {
            *line[original_span[i]] = temp_line[span[i]]
        }
    }
}

pub fn sort_pixels_in_line_new(settings: &Settings, line: &mut Vec<Rgba<u8>>) {
    let mut current_span: Vec<usize> = Vec::new();
    let mut spans: Vec<Vec<usize>> = Vec::new();

    for pixel in 0..line.len() {
        if settings.threshold.contains(
            settings
                .threshold
                .threshold_type
                .get_pixel_characteristic_value(&line[pixel]),
        ) {
            current_span.push(pixel);
        } else if !current_span.is_empty() {
            spans.push(mem::take(&mut current_span));
        }
    }

    if !current_span.is_empty() {
        spans.push(mem::take(&mut current_span));
    }

    let temp_line = line.clone();
    //i should definitely make this faster
    for span in &mut spans {
        let original_span = span.clone();
        span.sort_unstable_by(|a, b| settings.sort_by.compare(&line[*a], &line[*b]));

        for i in 0..span.len() {
            line[original_span[i]] = temp_line[span[i]];
        }
    }
}
