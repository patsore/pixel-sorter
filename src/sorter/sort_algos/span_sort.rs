use std::ops::Range;

use egui::{Color32, ComboBox, Slider, Ui};
use rayon::prelude::*;

use crate::sorter::Animateable;
use crate::sorter::sort_algos::SortMethod;

#[derive(Clone, Default)]
pub struct SpanSortMethod {
    pub config: SpanSortConfig,
}

#[derive(Clone)]
pub struct SpanSortConfig {
    pub(crate) threshold: Range<u8>,
    pub threshold_method: fn(&Color32) -> u8,
    threshold_method_name: String,
    pub(crate) invert_threshold: bool,

    pub sorting_method: fn(&Color32) -> u8,
    sorting_method_name: String,

    id: u32,
}

impl Default for SpanSortConfig {
    fn default() -> Self {
        SpanSortConfig {
            threshold: 0..255,
            threshold_method: average,
            threshold_method_name: "Average".to_string(),
            invert_threshold: false,

            sorting_method: average,
            sorting_method_name: "Average".to_string(),
            id: rand::random(),
        }
    }
}

impl SortMethod<Color32, ()> for SpanSortMethod {
    fn sort(&self, pixels: &mut [Color32]) {
        let spans = pixels.par_split_mut(|v| {
            let is_in_threshold = self.config.threshold.contains(&(self.config.threshold_method)(v));
            return if self.config.invert_threshold {
                is_in_threshold
            } else {
                !is_in_threshold
            };
        });

        spans.for_each(|span| {
            span.par_sort_unstable_by(|a, b| (self.config.sorting_method)(a).cmp(&(self.config.sorting_method)(b)))
        });
    }

    fn ui(&mut self, ui: &mut Ui) {
        let min = Slider::new(&mut self.config.threshold.start, 0..=255)
            .text("Lower bound of threshold")
            .drag_value_speed(0.1);
        ui.add(min);
        let max = Slider::new(&mut self.config.threshold.end, 0..=255)
            .text("Upper bound of threshold")
            .drag_value_speed(0.1);
        ui.add(max);


        let threshold_method = ComboBox::new(format!("threshold-{:?}", self.config.id), "Determine threshold value via")
            .selected_text(self.config.threshold_method_name.clone())
            .show_ui(ui, |ui| {
                {
                    let name = "Average";
                    if ui.selectable_value(&mut self.config.threshold_method, average, name).clicked() {
                        self.config.threshold_method_name = name.to_string();
                    }

                    let name = "Luminosity";
                    if ui.selectable_value(&mut self.config.threshold_method, luminosity, name).clicked() {
                        self.config.threshold_method_name = name.to_string();
                    }

                    let name = "Red";
                    if ui.selectable_value(&mut self.config.threshold_method, red, name).clicked() {
                        self.config.threshold_method_name = name.to_string();
                    }

                    let name = "Green";
                    if ui.selectable_value(&mut self.config.threshold_method, green, name).clicked() {
                        self.config.threshold_method_name = name.to_string();
                    }

                    let name = "Blue";
                    if ui.selectable_value(&mut self.config.threshold_method, blue, name).clicked() {
                        self.config.threshold_method_name = name.to_string();
                    }
                }
            });

        let sorting_method = ComboBox::new(format!("sort-{:?}", self.config.id), "Sort by")
            .selected_text(self.config.sorting_method_name.clone())
            .show_ui(ui, |ui| {
                {
                    let name = "Average";
                    if ui.selectable_value(&mut self.config.sorting_method, average, name).clicked() {
                        self.config.sorting_method_name = name.to_string();
                    }

                    let name = "Luminosity";
                    if ui.selectable_value(&mut self.config.sorting_method, luminosity, name).clicked() {
                        self.config.sorting_method_name = name.to_string();
                    }

                    let name = "Red";
                    if ui.selectable_value(&mut self.config.sorting_method, red, name).clicked() {
                        self.config.sorting_method_name = name.to_string();
                    }

                    let name = "Green";
                    if ui.selectable_value(&mut self.config.sorting_method, green, name).clicked() {
                        self.config.sorting_method_name = name.to_string();
                    }

                    let name = "Blue";
                    if ui.selectable_value(&mut self.config.sorting_method, blue, name).clicked() {
                        self.config.sorting_method_name = name.to_string();
                    }
                }
            });

        ui.checkbox(&mut self.config.invert_threshold, "Invert threshold range?");
    }
}

impl Animateable for SpanSortMethod {
    fn lerp(&mut self, target: &Self, weight: f32) {
        let config = &mut self.config;
        let target_config = &target.config;

        let new_threshold = {
            let threshold = &config.threshold;
            let target_threshold = &target_config.threshold;
            let start_dif = target_threshold.start as f32 - threshold.start as f32;
            let end_dif = target_threshold.end as f32 - threshold.end as f32;
            let new_start = (if start_dif.is_sign_positive() {
                (threshold.start as f32 + start_dif * weight).ceil()
            } else {
                (threshold.start as f32 + start_dif * weight).floor()
            }) as u8;
            let new_end = (if end_dif.is_sign_positive() {
                (threshold.end as f32 + end_dif * weight).ceil()
            } else {
                (threshold.end as f32 + end_dif * weight).floor()
            }) as u8;

            new_start..new_end
        };

        config.threshold = new_threshold;
    }
}

fn average(pixel: &Color32) -> u8 {
    let [r, g, b, _] = pixel.to_array();

    let average = r / 3 + g / 3 + b / 3;

    return average;
}

fn luminosity(pixel: &Color32) -> u8 {
    let [r, g, b, _] = pixel.to_array();

    let luminosity = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32);

    return luminosity.floor() as u8;
}


fn red(pixel: &Color32) -> u8 {
    return pixel.r();
}

fn green(pixel: &Color32) -> u8 {
    return pixel.g();
}

fn blue(pixel: &Color32) -> u8 {
    return pixel.b();
}

