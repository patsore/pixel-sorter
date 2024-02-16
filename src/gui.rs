use std::sync::mpsc::{channel, Receiver};
use std::thread;
use eframe::epaint::Color32;
use eframe::Frame;
use image::Rgba;
use egui::{ColorImage, ComboBox, Context, TextureFilter, TextureHandle, TextureId, TextureOptions};
use egui::load::SizedTexture;
use image::{DynamicImage, GenericImage};
use crate::sorter::{ScanlineSorter, Sorter, SpanSortConfig, SpanSortMethod};

#[derive(Default)]
pub struct AppState {
    image: Option<DynamicImage>,
    egui_image: Option<ColorImage>,
    image_handle: Option<TextureHandle>,
    change_receiver: Option<Receiver<PixelChanged>>,
    selected_line_algo: AvailableLineAlgos,
    selected_sort_algo: AvailableSortAlgos,
}

pub type PixelChanged = ((usize, usize), Rgba<u8>);

#[derive(Debug, PartialEq, Default)]
enum AvailableLineAlgos {
    #[default]
    Scanline,
}

#[derive(Debug, PartialEq, Default)]
enum AvailableSortAlgos {
    #[default]
    SpanSort,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let image = image::open("image.jpeg").unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );

        Self {
            image: Some(image),
            egui_image: Some(color_image),
            ..Default::default()
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        if let Some(color_image) = self.egui_image.clone() {
            self.image_handle.get_or_insert_with(|| {
                ctx.load_texture("image", color_image.clone(), TextureOptions {
                    magnification: TextureFilter::Nearest,
                    minification: TextureFilter::Nearest,
                    wrap_mode: Default::default(),
                })
            });
        }

        egui::SidePanel::left("settings_panel").show(ctx, |ui| {
            let ref mut line_algo = self.selected_line_algo;
            ComboBox::from_label("Line algorithm")
                .selected_text(format!("{:?}", line_algo))
                .show_ui(ui, |ui| {
                    ui.selectable_value(line_algo, AvailableLineAlgos::Scanline, "Horizontal Lines");
                });

            let ref mut sort_algo = self.selected_sort_algo;
            ComboBox::from_label("Sorting algorithm")
                .selected_text(format!("{:?}", sort_algo))
                .show_ui(ui, |ui| {
                    ui.selectable_value(sort_algo, AvailableSortAlgos::SpanSort, "Sort against a threshold");
                });


            if ui.button("Do").clicked() {
                if let Some(ref mut texture) = self.image_handle {
                    let (sender, receiver) = channel::<PixelChanged>();
                    // self.change_receiver = Some(receiver);
                    let mut recv_tex = texture.clone();
                    thread::spawn(move || {
                        let mut i = 0;
                        while let Ok(value) = receiver.recv() {
                            i += 1;
                            let [r, g, b, a] = value.1.0;
                            recv_tex.set_partial([value.0.0, value.0.1], ColorImage::new([1, 1], Color32::from_rgba_unmultiplied(r, g, b, a)), Default::default());
                            // println!("{:?}", value)
                        }
                    });
                    let sort_algorithm = Box::new(match sort_algo {
                        AvailableSortAlgos::SpanSort => {
                            SpanSortMethod {
                                config: SpanSortConfig {
                                    threshold: 0..255,
                                },
                                sender,
                            }
                        }
                        _ => {
                            panic!("Unimplemented sorting algorithm!")
                        }
                    });
                    let line_algorithm = match line_algo {
                        AvailableLineAlgos::Scanline => {
                            let sorter = ScanlineSorter;
                            let sorter_image = self.image.clone().unwrap();
                            thread::spawn(move || {
                                sorter.sort_image(&sorter_image, sort_algorithm);
                            });
                        }
                    };
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref mut texture) = self.image_handle {
                let width = f32::min(ui.available_width(), ui.available_height() * texture.aspect_ratio());
                let size = egui::vec2(width, width / texture.aspect_ratio());


                ui.image(
                    SizedTexture {
                        id: TextureId::from(texture),
                        size,
                    }).mark_changed();
            }
        },
        );
        ctx.request_repaint();
    }
}

