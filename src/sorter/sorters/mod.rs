mod scanline;
mod angled;

use std::fmt::{Debug, Formatter};
use eframe::epaint::{Color32, ColorImage};
use egui::Ui;
pub use scanline::*;
pub use angled::*;
use crate::sorter::{SortMethod};

//T represents our pixels, A represents the image.
// S represents the sorter that we'll use.
// R represents the return we want.

pub trait Sorter<PixelType, ImageType, ReturnType, SortReturnType>
{
    fn sort_image(&self, image: ImageType, sorter: impl SortMethod<PixelType, SortReturnType>) -> ReturnType;

    fn ui(&mut self, ui: &mut Ui);
}

#[derive(Clone)]
pub enum AvailableLineAlgos {
    Scanline(ScanlineSorter),
    Angled(AngledSorter),
}

impl Sorter<Color32, &mut ColorImage, (), ()> for AvailableLineAlgos{
    fn sort_image(&self, image: &mut ColorImage, sorter: impl SortMethod<Color32, ()>) -> () {
        match self{
            AvailableLineAlgos::Scanline(line_alg) => {
                line_alg.sort_image(image, sorter);
            }
            AvailableLineAlgos::Angled(line_alg) => {
                line_alg.sort_image(image, sorter)
            }
        }
    }

    fn ui(&mut self, ui: &mut Ui) {
        match self{
            AvailableLineAlgos::Scanline(line_alg) => {
                line_alg.ui(ui);
            }
            AvailableLineAlgos::Angled(line_alg) => {
                line_alg.ui(ui)
            }
        }
    }
}

impl Default for AvailableLineAlgos{
    fn default() -> Self {
        Self::Scanline(ScanlineSorter)
    }
}

impl Debug for AvailableLineAlgos{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let variant_name = match self{
            AvailableLineAlgos::Scanline(_) => {
                "ScanLine"
            }
            AvailableLineAlgos::Angled(_) => {
                "AngledLine"
            }
        };
        write!(f, "{}", variant_name)
    }
}

impl PartialEq for AvailableLineAlgos{
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

