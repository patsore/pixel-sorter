#![feature(get_many_mut)]
#![feature(file_create_new)]

use std::num::NonZeroU32;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use fast_image_resize as fr;
use fast_image_resize::{Image, PixelType};
use rayon::prelude::*;
use softbuffer::Buffer;
use winit::keyboard::{Key, NamedKey};

#[tokio::main]
async fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Pixel Sorter")
        .build(&event_loop)
        .unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let mut sorter = Sorter::new("./image.jpeg");
    let display_image = sorter.display_image.clone();
    let current_image = sorter.current_image.clone();
    let window_redrawer = window.clone();
    let sorter_display_image = display_image.clone();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;


        Sorter::sort_buffer(current_image, sorter_display_image, window_redrawer);

    });


    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::RedrawRequested,
                } if window_id == window.id() => {
                    if let (Some(width), Some(height)) = {
                        let size = window.inner_size();
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    } {
                        surface.resize(width, height).unwrap();

                        let mut buffer = surface.buffer_mut().unwrap();

                        Sorter::draw_buffer(display_image.clone(), &mut buffer, width.get(), height.get(), sorter.aspect_ratio.clone());

                        buffer.present().unwrap();
                    }
                }
                Event::WindowEvent {
                    event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                        ..
                    },
                    window_id,
                } if window_id == window.id() => {
                    elwt.exit();
                }
                _ => {}
            }
        })
        .unwrap();
}


struct Sorter<'a> {
    pub image_width: usize,
    pub image_height: usize,

    pub aspect_ratio: f32,

    // pub aspect_ratio: f32,
    //the amount of channels in the buffer
    pub channel_count: u8,

    //buffer with the original image contents
    pub original_image: Vec<u8>,

    //buffer with the current image state
    pub current_image: Arc<Mutex<Image<'a>>>,

    pub display_image: Arc<Mutex<Image<'a>>>,
}

impl Sorter<'_> {
    pub fn new(path: &str) -> Self {
        let image = image::open(path).unwrap().into_rgb8();
        let image_buffer = image.as_raw();
        let (width, height) = image.dimensions();
        //the image used to actually display stuff on-screen
        let display_image = Arc::new(Mutex::new(fr::Image::from_vec_u8(
            NonZeroU32::try_from(width).unwrap(),
            NonZeroU32::try_from(height).unwrap(),
            image_buffer.clone(),
            PixelType::U8x3,
        ).unwrap()));

        //the image that will store the state of the image
        let current_image = Arc::new(Mutex::new(fr::Image::from_vec_u8(
            NonZeroU32::try_from(width).unwrap(),
            NonZeroU32::try_from(height).unwrap(),
            image_buffer.clone(),
            PixelType::U8x3,
        ).unwrap()));

        Self {
            image_width: width as usize,
            image_height: height as usize,
            aspect_ratio: width as f32 / height as f32,
            channel_count: 3,
            original_image: image_buffer.clone(),
            current_image,
            display_image,
        }
    }


    pub fn reset_current_image(&mut self) {
        let mut current_image = self.current_image.lock().unwrap();
        current_image.buffer_mut().iter().clone_from(&self.original_image.iter());
        drop(current_image);
        self.update_displayed_image();
    }

    pub fn update_displayed_image(&mut self) {
        if let Ok(mut display_image) = self.display_image.try_lock() {
            if let Ok(mut current_image) = self.current_image.try_lock() {
                display_image.buffer_mut().iter().clone_from(&current_image.buffer().iter());
            }
        }
    }


    pub fn draw_buffer(display_image: Arc<Mutex<Image>>, mut buffer: &mut Buffer<Arc<Window>, Arc<Window>>, width: u32, height: u32, aspect_ratio: f32) {
        //clear the buffer
        buffer.fill(0);

        let display_image = display_image.lock().unwrap();

        let dst_ratio = width as f32 / height as f32;
        let (resize_width, resize_height) = if aspect_ratio > dst_ratio {
            (width, (width as f32 / aspect_ratio).floor() as u32)
        } else {
            ((height as f32 * aspect_ratio).floor() as u32, height)
        };
        let start_x = (width - resize_width) / 2;
        let start_y = (height - resize_height) / 2;

        let mut resized_image = fr::Image::new(
            NonZeroU32::new(resize_width).unwrap(),
            NonZeroU32::new(resize_height).unwrap(),
            display_image.pixel_type(),
        );
        let mut resizer = fr::Resizer::new(
            fr::ResizeAlg::Nearest,
        );
        let mut resized_view = resized_image.view_mut();
        resizer.resize(&display_image.view(), &mut resized_view).unwrap();

        for (i, pixel_data) in resized_image.buffer().chunks(3).enumerate() {
            let r = pixel_data[0] as u32;
            let g = pixel_data[1] as u32;
            let b = pixel_data[2] as u32;

            let x = start_x + i as u32 % resize_width;
            let y = start_y + i as u32 / resize_width;

            let index = y * width + x;
            // println!("{:?}, {:?}", x, y);
            buffer[index as usize] = (r << 16 | (g << 8) | (b));
        }
    }

    pub fn sort_buffer(current_image_arc: Arc<Mutex<Image>>, display_image_arc: Arc<Mutex<Image>>, window_redrawer: Arc<Window>) {
        let mut current_image = current_image_arc.lock().unwrap();
        let img_width = current_image.width().get().clone();
        current_image.buffer_mut().par_chunks_mut(img_width as usize * 3)
            .enumerate()
            .for_each(|(i, line)|
                {
                    let mut current_span_start: Option<usize> = None;
                    let mut spans: Vec<Range<usize>> = Vec::new();
                    let mut pixel_values: Vec<u8> = Vec::with_capacity(img_width as usize);
                    line.chunks(3).enumerate().for_each(|(i, pixel)| {
                        let pixel = (pixel[0].clone(), pixel[1].clone(), pixel[2].clone());
                        let pixel_value = ((pixel.0 as f32 + pixel.1 as f32 + pixel.2 as f32) / 3_f32).floor() as u8;
                        pixel_values.push(pixel_value);
                        if pixel_value < 255 {
                            if current_span_start.is_none() {
                                current_span_start = Some(i);
                            }
                        } else if let Some(current_span_start) = current_span_start.take() {
                            spans.push(current_span_start..i);
                        }
                    });

                    if let Some(current_span_start) = current_span_start.take() {
                        spans.push(current_span_start..img_width as usize);
                        let _last_span = spans.last().unwrap();
                    }

                    let temp_line = line.iter().map(|i| i.clone()).collect::<Vec<_>>();
                    for span in spans {
                        let mut span_pixel_values = span.clone().map(|i| {
                            (i.clone(), pixel_values[i.clone()])
                        }).collect::<Vec<_>>();

                        span_pixel_values.sort_unstable_by(|(_, pixel_value_a), (_, pixel_value_b)| {
                            pixel_value_a.cmp(pixel_value_b)
                        });


                        for (span_i, (value_i, _)) in span.zip(span_pixel_values) {
                            line[span_i * 3] = temp_line[value_i * 3];

                            line[span_i * 3 + 1] = temp_line[value_i * 3 + 1];

                            line[span_i * 3 + 2] = temp_line[value_i * 3 + 2];
                        }
                    }
                    let mut display_image = display_image_arc.lock().unwrap();
                    let mut display_buffer = display_image.buffer_mut();
                    let offset = i * img_width as usize * 3;
                    for (i_line, value) in line.iter().enumerate(){
                        display_buffer[offset + i_line] = *value
                    }
                    drop(display_image);
                    window_redrawer.request_redraw();
                }
            );
    }
}