use image::{GenericImage, GenericImageView};
use crate::gui::AppState;
use crate::sorter::{Sorter};

mod gui;

mod sorter;

fn main() {
    let mut native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Pixel Sorter",
        native_options,
        Box::new(|cc| Box::new(AppState::new(cc))),
    ).expect("TODO: panic message");
}
