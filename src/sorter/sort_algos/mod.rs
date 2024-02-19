mod span_sort;

use std::fmt::{Debug, Formatter};
use eframe::epaint::Color32;
use egui::Ui;
pub use span_sort::*;

//T is the type that represents a pixel
//A represents how we want data to be returned.
pub trait SortMethod<P, R>: Sync + Clone {
    fn sort(&self, pixels: &mut [P]) -> R;

    fn ui(&mut self, ui: &mut Ui);
}

#[derive(Clone)]
pub enum AvailableSortAlgos {
    SpanSort(SpanSortMethod),
}

impl SortMethod<Color32, ()> for AvailableSortAlgos {
    fn sort(&self, pixels: &mut [Color32]) -> () {
        match self {
            AvailableSortAlgos::SpanSort(sort_algo) => {
                sort_algo.sort(pixels)
            }
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        match self {
            AvailableSortAlgos::SpanSort(sort_algo) => {
                sort_algo.ui(ui)
            }
        }
    }
}

impl Default for AvailableSortAlgos {
    fn default() -> Self {
        Self::SpanSort(SpanSortMethod {
            config: SpanSortConfig { threshold: 0..255, invert_threshold: false },
        })
    }
}

impl Debug for AvailableSortAlgos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self {
            AvailableSortAlgos::SpanSort(_) => {
                "SpanSort"
            }
        };
        write!(f, "{}", variant_name)
    }
}

impl PartialEq for AvailableSortAlgos {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}