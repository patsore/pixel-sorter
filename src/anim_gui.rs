use std::thread;

use eframe::epaint::{Color32, Stroke};
use egui::{Button, ComboBox, Context, Layout, Margin, vec2};

use crate::gui::new_config_frame;
use crate::sorter::{AngledSorter, Animateable, AvailableLineAlgos, AvailableSortAlgos, ScanlineSorter, Sorter, SortMethod};

#[derive(Default)]
pub struct AnimationState {
}

impl AnimationState {
    pub fn ui(&mut self, ctx: &Context) {

    }
}