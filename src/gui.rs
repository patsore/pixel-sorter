use std::path::PathBuf;
use std::thread;
use std::time::Instant;
use eframe::Frame;
use egui::{Button, Checkbox, Color32, ColorImage, ComboBox, Context, Direction, Layout, Stroke, TextureFilter, TextureHandle, TextureId, TextureOptions, Vec2};
use egui::load::SizedTexture;
use egui::panel::TopBottomSide;
use image::{DynamicImage};
use crate::sorter::{ScanlineSorter, Sorter, SortMethod, SpanSortConfig, SpanSortMethod};
use crate::sorter::{AvailableLineAlgos, AvailableSortAlgos};

#[derive(Default)]
pub struct AppState {
    image: Option<DynamicImage>,
    egui_image: Option<ColorImage>,
    image_handle: Option<TextureHandle>,
    selected_line_algo: AvailableLineAlgos,
    selected_sort_algo: AvailableSortAlgos,
    live_sort: bool,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let image = image::open("output.jpeg").unwrap();
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

    pub fn open_image(&mut self, path_buf: PathBuf) {
        let image = image::open(path_buf).unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );
        if let Some(ref mut texture) = self.image_handle {
            texture.set(color_image.clone(), Default::default())
        }
        self.egui_image = Some(color_image);
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Some(color_image) = self.egui_image.clone() {
            self.image_handle.get_or_insert_with(|| {
                ctx.load_texture("image", color_image.clone(), TextureOptions {
                    magnification: TextureFilter::Nearest,
                    minification: TextureFilter::Nearest,
                    wrap_mode: Default::default(),
                })
            });
        }

        egui::TopBottomPanel::new(TopBottomSide::Top, "general_controls").show(ctx, |ui|
            {
                if ui.button("Open").clicked() {
                    let task = rfd::FileDialog::new().pick_file();
                    if let Some(file) = task {
                        self.open_image(file);
                    }
                }
            });

        egui::SidePanel::left("settings_panel").show(ctx, |ui| {
            ui.with_layout(Layout::default(), |ui| {
                let ref mut line_algo = self.selected_line_algo;
                let ref mut sort_algo = self.selected_sort_algo;
                ComboBox::from_label("Line algorithm")
                    .selected_text(format!("{:?}", line_algo))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(line_algo, AvailableLineAlgos::Scanline, "Horizontal Lines");
                    });

                let line_algorithm = match line_algo {
                    AvailableLineAlgos::Scanline => {
                        ScanlineSorter
                    }
                };

                ui.separator();

                ComboBox::from_label("Sorting algorithm")
                    .selected_text(format!("{:?}", sort_algo))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(sort_algo, AvailableSortAlgos::SpanSort(Default::default()), "Sort against a threshold");
                    });

                let mut sort_algorithm = match sort_algo {
                    AvailableSortAlgos::SpanSort(sort_algorithm) => {
                        sort_algorithm
                    }
                };

                egui::Frame::none()
                    .fill(Color32::from_additive_luminance(15))
                    .rounding(10.0)
                    .outer_margin(5.0)
                    .inner_margin(10.0)
                    .stroke(Stroke::from((1.0, Color32::from_additive_luminance(20))))
                    .show(ui, |ui| {
                        sort_algorithm.ui(ui);
                        ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                    });

                let mut option_line_alg = Some(line_algorithm);

                ui.add_enabled_ui(!self.live_sort, |ui| {
                    let button = Button::new("Sort!").min_size(Vec2::new(ui.available_width(), 10.0));
                    if ui.add_sized(egui::vec2(ui.available_width(), 10.0), button).clicked() {
                        if let Some(ref mut texture) = self.image_handle {
                            if let Some(line_algorithm) = option_line_alg.take() {
                                let mut sorter_image = self.egui_image.clone().unwrap();
                                let mut texture_handle = self.image_handle.clone().unwrap();
                                let t_sort_alg = sort_algorithm.clone();
                                thread::spawn(move || {
                                    let start = Instant::now();

                                    line_algorithm.sort_image(&mut sorter_image, t_sort_alg);
                                    texture_handle.set(sorter_image, Default::default());

                                    println!("Sorting took {:?}", start.elapsed());
                                });
                            }
                        }
                    }
                });

                let checkbox = Checkbox::new(&mut self.live_sort, "Sort Live?");
                ui.add_sized(egui::vec2(100.0, 10.0), checkbox);
                if self.live_sort {
                    if let Some(line_algorithm) = option_line_alg.take() {
                        let mut sorter_image = self.egui_image.clone().unwrap();
                        let mut texture_handle = self.image_handle.clone().unwrap();
                        let t_sort_alg = sort_algorithm.clone();
                        thread::spawn(move || {
                            line_algorithm.sort_image(&mut sorter_image, t_sort_alg);
                            texture_handle.set(sorter_image, Default::default());
                        });
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref mut texture) = self.image_handle {
                let width = f32::min(ui.available_width(), ui.available_height() * texture.aspect_ratio());
                let size = egui::vec2(width, width / texture.aspect_ratio());

                ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                    ui.image(
                        SizedTexture {
                            id: TextureId::from(texture),
                            size,
                        }).mark_changed();
                });
            }
        },
        );
        ctx.request_repaint();
    }
}

