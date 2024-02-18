use std::ops::Range;
use egui::{Color32, Slider, Ui};
use crate::sorter::sort_algos::SortMethod;
use rayon::prelude::*;

#[derive(Clone, Default)]
pub struct SpanSortMethod {
    pub config: SpanSortConfig,
}

#[derive(Clone, Default)]
pub struct SpanSortConfig {
    pub(crate) threshold: Range<u8>,
    pub(crate) invert_threshold: bool,
}

impl SortMethod<Color32, ()> for SpanSortMethod {
    fn sort(&self, pixels: &mut [Color32]) {
        let spans = pixels.split_mut(|v| {
            let is_in_threshold = self.config.threshold.contains(&threshold_method(v));
            return if self.config.invert_threshold { is_in_threshold } else { !is_in_threshold };
        });

        spans.par_bridge().for_each(|span| {
            span.sort_unstable_by(|a, b| {
                sorting_method(a).cmp(&sorting_method(b))
            })
        });
    }

    fn ui(&mut self, ui: &mut Ui) {

        let min = Slider::new(&mut self.config.threshold.start, 0..=255).text("Lower bound of threshold").drag_value_speed(0.1);
        ui.add(min);
        let max = Slider::new(&mut self.config.threshold.end, 0..=255).text("Upper bound of threshold").drag_value_speed(0.1);
        ui.add(max);

        ui.checkbox(&mut self.config.invert_threshold, "Invert threshold range?");
    }
}

pub fn threshold_method(pixel: &Color32) -> u8 {
    let [r, g, b, _] = pixel.to_array();

    let average = r / 3 + g / 3 + b / 3;

    return average;
}

pub fn sorting_method(pixel: &Color32) -> u8 {
    let [r, g, b, _] = pixel.to_array();

    let average = r / 3 + g / 3 + b / 3;

    return average;
}