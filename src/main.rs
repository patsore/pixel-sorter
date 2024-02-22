#![feature(slice_split_at_unchecked, vec_into_raw_parts)]

use crate::gui::AppState;

mod gui;
mod sorter;

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Pixel Sorter",
        native_options,
        Box::new(|cc| Box::new(AppState::new(cc))),
    )
    .expect("TODO: panic message");
}
