use std::path::PathBuf;
use std::thread;
use std::time::Instant;

use eframe::emath::vec2;
use eframe::Frame;
use egui::{
    Button, Checkbox, Color32, ColorImage, ComboBox, Context, Direction, Layout, Margin, Stroke,
    TextureFilter, TextureHandle, TextureId, TextureOptions, Vec2,
};
use egui::load::SizedTexture;
use egui::panel::TopBottomSide;
use image::{ImageBuffer, Rgba};

use crate::sorter::{AngledSorter, Animateable, ScanlineSorter, Sorter, SortMethod};
use crate::sorter::{AvailableLineAlgos, AvailableSortAlgos};

#[derive(Default)]
pub struct AppState {
    original_image: Option<ColorImage>,
    working_image: Option<ColorImage>,
    image_handle: Option<TextureHandle>,
    selected_line_algo: AvailableLineAlgos,
    selected_sort_algo: AvailableSortAlgos,
    live_sort: bool,
    anim_mode: bool,
    pub sort_keyframes: Vec<AvailableSortAlgos>,
    pub line_keyframes: Vec<AvailableLineAlgos>,
}

impl AppState {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // let image = image::open("output.jpeg").unwrap();
        // let size = [image.width() as _, image.height() as _];
        // let image_buffer = image.to_rgba8();
        // let pixels = image_buffer.as_flat_samples();
        // let color_image = ColorImage::from_rgba_unmultiplied(
        //     size,
        //     pixels.as_slice(),
        // );

        // Self {
        //     image: Some(image),
        //     original_image: Some(color_image.clone()),
        //     working_image: Some(color_image),
        //     ..Default::default()
        // }
        Self {
            sort_keyframes: vec![],
            line_keyframes: vec![],
            ..Default::default()
        }
    }

    pub fn save_image(&mut self, path_buf: PathBuf) {
        if let Some(ref mut image) = self.working_image {
            let [w, h] = image.size;
            let pixels = image.as_raw();

            let dyn_image =
                ImageBuffer::<Rgba<u8>, &[u8]>::from_raw(w as u32, h as u32, pixels).unwrap();
            dyn_image.save(path_buf).unwrap();
        }
    }

    pub fn open_image(&mut self, path_buf: PathBuf) {
        let image = image::open(path_buf).unwrap();
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        if let Some(ref mut texture) = self.image_handle {
            texture.set(color_image.clone(), Default::default())
        }
        self.original_image = Some(color_image.clone());
        self.working_image = Some(color_image);
    }

    pub fn load_texture(&mut self, ctx: &Context) {
        if let Some(color_image) = self.original_image.clone() {
            self.image_handle.get_or_insert_with(|| {
                ctx.load_texture(
                    "image",
                    color_image.clone(),
                    TextureOptions {
                        magnification: TextureFilter::Nearest,
                        minification: TextureFilter::Nearest,
                        wrap_mode: Default::default(),
                    },
                )
            });
        }
    }

    pub fn sorter_ui(&mut self, ctx: &Context) {
        egui::SidePanel::left("settings_panel").show(ctx, |ui| {
            ui.with_layout(Layout::default(), |ui| {
                ui.add_space(3.0);
                let ref mut line_algo = self.selected_line_algo;
                let ref mut sort_algo = self.selected_sort_algo;

                ComboBox::from_label("Line algorithm")
                    .selected_text(format!("{:?}", line_algo))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            line_algo,
                            AvailableLineAlgos::Scanline(ScanlineSorter),
                            "Horizontal Lines",
                        );
                        ui.selectable_value(
                            line_algo,
                            AvailableLineAlgos::Angled(AngledSorter { angle: 0.0 }),
                            "Angled Lines",
                        );
                    });

                new_config_frame().show(ui, |ui| {
                    line_algo.ui(ui);
                    ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                });

                ui.separator();

                ComboBox::from_label("Sorting algorithm")
                    .selected_text(format!("{:?}", sort_algo))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            sort_algo,
                            AvailableSortAlgos::SpanSort(Default::default()),
                            "Sort against a threshold",
                        );
                    });

                new_config_frame().show(ui, |ui| {
                    sort_algo.ui(ui);
                    ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                });

                let mut option_line_alg = Some(line_algo.clone());
                let mut sorter_image = self.working_image.as_mut();

                ui.add_enabled_ui(!self.live_sort, |ui| {
                    let button =
                        Button::new("Sort!").min_size(Vec2::new(ui.available_width(), 10.0));
                    if ui
                        .add_sized(egui::vec2(ui.available_width(), 10.0), button)
                        .clicked()
                    {
                        if let Some(mut texture_handle) = self.image_handle.clone() {
                            if let Some(line_algorithm) = option_line_alg.take() {
                                if let Some(mut sorter_image) = sorter_image.take() {
                                    let t_sort_alg = sort_algo.clone();
                                    thread::scope(|scope| {
                                        scope.spawn(move || {
                                            let start = Instant::now();

                                            line_algorithm
                                                .sort_image(&mut sorter_image, t_sort_alg);
                                            texture_handle
                                                .set(sorter_image.clone(), Default::default());

                                            println!("Sorting took {:?}", start.elapsed());
                                        });
                                    });
                                }
                            }
                        }
                    }
                });

                let checkbox = Checkbox::new(&mut self.live_sort, "Live Preview?");
                ui.add_sized(egui::vec2(100.0, 10.0), checkbox);
                if self.live_sort {
                    if let Some(line_algorithm) = option_line_alg.take() {
                        if let Some(sorter_image) = sorter_image.take() {
                            let mut texture_handle = self.image_handle.clone().unwrap();
                            let t_sort_alg = sort_algo.clone();
                            //cloning this because otherwise live sorting will just mess the image up immediately.
                            let mut sorter_image = sorter_image.clone();
                            thread::scope(|scope| {
                                scope.spawn(move || {
                                    line_algorithm.sort_image(&mut sorter_image, t_sort_alg);
                                    texture_handle.set(sorter_image.clone(), Default::default());
                                });
                            });
                        }
                    }
                }
            });
        });
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.load_texture(ctx);

        egui::TopBottomPanel::new(TopBottomSide::Top, "general_controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    let task = rfd::FileDialog::new().pick_file();
                    if let Some(file) = task {
                        self.open_image(file);
                    }
                }

                if ui.button("Save").clicked() {
                    let task = rfd::FileDialog::new().save_file();
                    if let Some(file) = task {
                        self.save_image(file);
                    }
                }

                if ui.button("Reset Image").clicked() {
                    if let Some(ref mut texture) = self.image_handle {
                        if let Some(ref mut image) = self.original_image {
                            texture.set(image.clone(), Default::default());
                            self.working_image = Some(image.clone());
                        }
                    }
                }
                if ui.button("Toggle animation mode").clicked() {
                    self.anim_mode = !self.anim_mode;
                }
            });
        });

        if !self.anim_mode {
            self.sorter_ui(ctx)
        } else {
            egui::SidePanel::left("settings_panel").show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 50.0)
                    .show(ui, |ui| {
                        ui.with_layout(Layout::default(), |ui| {
                            ui.add_space(3.0);
                            let mut to_remove = Vec::new();
                            for (i, (line_algo, sort_algo)) in self
                                .line_keyframes
                                .iter_mut()
                                .zip(self.sort_keyframes.iter_mut())
                                .enumerate()
                            {
                                ui.add_space(3.0);
                                egui::Frame::none()
                                    .outer_margin(Margin::from(egui::vec2(10.0, 0.0)))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("Frame {i}"));

                                            let rm_frame_button =
                                                Button::new("x").stroke(Stroke::from((
                                                    0.5,
                                                    Color32::from_additive_luminance(30),
                                                )));
                                            let button_response = ui.add(rm_frame_button);
                                            if button_response.clicked() {
                                                to_remove.push(i);
                                            }
                                            button_response.on_hover_text("Remove this frame");
                                        });
                                    });

                                egui::Frame::none()
                                    .rounding(10.0)
                                    .outer_margin(5.0)
                                    .inner_margin(10.0)
                                    .stroke(Stroke::from((
                                        1.0,
                                        Color32::from_additive_luminance(30),
                                    )))
                                    .show(ui, |ui| {
                                        ComboBox::new(format!("{i}-linalg"), "Line algorithm")
                                            .selected_text(format!("{:?}", line_algo))
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    line_algo,
                                                    AvailableLineAlgos::Scanline(ScanlineSorter),
                                                    "Horizontal Lines",
                                                );
                                                ui.selectable_value(
                                                    line_algo,
                                                    AvailableLineAlgos::Angled(AngledSorter {
                                                        angle: 0.0,
                                                    }),
                                                    "Angled Lines",
                                                );
                                            });

                                        new_config_frame().show(ui, |ui| {
                                            line_algo.ui(ui);
                                            ui.allocate_space(egui::vec2(
                                                ui.available_width(),
                                                0.0,
                                            ));
                                        });

                                        ui.separator();

                                        ComboBox::new(format!("{i}-sortalg"), "Sort algorithm")
                                            .selected_text(format!("{:?}", sort_algo))
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    sort_algo,
                                                    AvailableSortAlgos::SpanSort(Default::default()),
                                                    "Sort against a threshold",
                                                );
                                            });

                                        new_config_frame().show(ui, |ui| {
                                            sort_algo.ui(ui);
                                            ui.allocate_space(egui::vec2(
                                                ui.available_width(),
                                                0.0,
                                            ));
                                        });
                                    });
                            }

                            for i in to_remove {
                                self.sort_keyframes.remove(i);
                                self.line_keyframes.remove(i);
                            }

                            let add_keyframe_button = Button::new("+")
                                .stroke(Stroke::from((0.5, Color32::from_additive_luminance(30))));

                            if ui
                                .add_sized(vec2(ui.available_width(), 10.0), add_keyframe_button)
                                .clicked()
                            {
                                self.sort_keyframes.push(AvailableSortAlgos::default());
                                self.line_keyframes.push(AvailableLineAlgos::default());
                            }
                        });
                    });

                ui.separator();

                let preview_button = Button::new("Preview");
                if ui
                    .add_sized(egui::vec2(ui.available_width(), 10.0), preview_button)
                    .clicked()
                {
                    if let Some(mut texture) = self.image_handle.clone() {
                        let mut sort_keyframes = self.sort_keyframes.clone();
                        let mut line_keyframes = self.line_keyframes.clone();

                        let image = self.working_image.clone().unwrap();

                        thread::spawn(move || {
                            let mut current_sort = sort_keyframes.remove(0);
                            let mut current_line = line_keyframes.remove(0);
                            while !line_keyframes.is_empty() {
                                let mut target_sort = sort_keyframes.remove(0);
                                let mut target_line = line_keyframes.remove(0);
                                {
                                    while current_sort != target_sort {
                                        current_sort = target_sort;
                                        target_sort = sort_keyframes.remove(0);
                                    }
                                    while current_line != target_line {
                                        current_line = target_line;
                                        target_line = line_keyframes.remove(0);
                                    }
                                }
                                let frames = 180.0;
                                for _ in 0..frames as u16 {
                                    let mut sorting_image = image.clone();
                                    current_sort.lerp(&target_sort, 0.01);
                                    current_line.lerp(&target_line, 0.01);
                                    current_line
                                        .sort_image(&mut sorting_image, current_sort.clone());
                                    texture.set(sorting_image.clone(), Default::default());
                                }
                            }
                        });
                    }
                }

                let sort_button = Button::new("Sort");

                if ui
                    .add_sized(egui::vec2(ui.available_width(), 10.0), sort_button)
                    .clicked()
                {
                    let task = rfd::FileDialog::new().pick_folder();
                    if let Some(folder) = task {
                        if let Some(mut texture) = self.image_handle.clone() {
                            let mut sort_keyframes = self.sort_keyframes.clone();
                            let mut line_keyframes = self.line_keyframes.clone();

                            let image = self.working_image.clone().unwrap();

                            thread::spawn(move || {
                                let mut file_name = 0;
                                let mut current_sort = sort_keyframes.remove(0);
                                let mut current_line = line_keyframes.remove(0);
                                while !line_keyframes.is_empty() {
                                    let mut target_sort = sort_keyframes.remove(0);
                                    let mut target_line = line_keyframes.remove(0);
                                    {
                                        while current_sort != target_sort {
                                            current_sort = target_sort;
                                            target_sort = sort_keyframes.remove(0);
                                        }
                                        while current_line != target_line {
                                            current_line = target_line;
                                            target_line = line_keyframes.remove(0);
                                        }
                                    }
                                    let frames = 180.0;
                                    for _ in 0..frames as u16 {
                                        let mut sorting_image = image.clone();
                                        current_sort.lerp(&target_sort, 1.0 / frames);
                                        current_line.lerp(&target_line, 1.0 / frames);
                                        current_line
                                            .sort_image(&mut sorting_image, current_sort.clone());
                                        texture.set(sorting_image.clone(), Default::default());
                                        let file = folder.clone().join(format!("{:0>5}.png", file_name));

                                        let [w, h] = sorting_image.size;
                                        let pixels = sorting_image.as_raw();

                                        let dyn_image =
                                            ImageBuffer::<Rgba<u8>, &[u8]>::from_raw(w as u32, h as u32, pixels).unwrap();
                                        dyn_image.save(file).unwrap();


                                        file_name += 1;
                                    }
                                }
                            });
                        }
                    }
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref mut texture) = self.image_handle {
                let width = f32::min(
                    ui.available_width(),
                    ui.available_height() * texture.aspect_ratio(),
                );
                let size = egui::vec2(width, width / texture.aspect_ratio());

                ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                    ui.image(SizedTexture {
                        id: TextureId::from(texture),
                        size,
                    })
                        .mark_changed();
                });
            }
        });
        ctx.request_repaint();
    }
}

pub fn new_config_frame() -> egui::containers::Frame {
    egui::Frame::none()
        .fill(Color32::from_additive_luminance(15))
        .rounding(10.0)
        .outer_margin(5.0)
        .inner_margin(10.0)
        .stroke(Stroke::from((1.0, Color32::from_additive_luminance(20))))
}
