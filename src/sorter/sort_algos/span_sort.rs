use std::mem;
use std::ops::Range;
use flume::Sender;
use image::Rgba;
use crate::gui::{PixelChanged, SpanChanged};
use crate::sorter::sort_algos::SortMethod;
pub struct SpanSortMethod {
    pub config: SpanSortConfig,
    pub sender: Option<Sender<SpanChanged>>,
}
pub struct SpanSortConfig {
    pub(crate) threshold: Range<u8>,
}

impl SortMethod<Rgba<u8>, ()> for SpanSortMethod {
    fn sort(&self, pixels: Vec<Rgba<u8>>, line: usize) {
        let mut spans = vec![];
        let mut current_span_start: Option<Vec<_>> = None;

        for (i, pixel) in pixels.iter().enumerate() {
            if self.config.threshold.contains(&threshold_method(pixel)) {
                if let Some(ref mut current_span) = current_span_start {
                    current_span.push((i, pixel.clone()));
                } else {
                    current_span_start = Some(vec![(i, pixel.clone())]);
                }
            } else if current_span_start.is_some() {
                let span_start = mem::replace(&mut current_span_start, None);
                spans.push(span_start.unwrap());
            }
        }

        if current_span_start.is_some() {
            let span_start = mem::replace(&mut current_span_start, None);
            spans.push(span_start.unwrap());
        }

        spans.iter_mut().for_each(|span| {
            let original_span = span.clone();
            span.sort_unstable_by(|a, b| {
                sorting_method(&a.1).cmp(&sorting_method(&b.1))
            });
            let span_changed = original_span.iter().enumerate().map(|(i, v)| {
                PixelChanged::from(((v.0, line), span[i].1))
            }).collect::<Vec<_>>();
            self.sender.as_ref().unwrap().send(span_changed).unwrap();
        });
    }

}

pub fn threshold_method(pixel: &Rgba<u8>) -> u8 {
    let [r, g, b, a] = pixel.0;

    let average = r / 3 + g / 3 + b / 3;

    return average;
}

pub fn sorting_method(pixel: &Rgba<u8>) -> u8{
    let [r, g, b, a] = pixel.0;

    let average = r / 3 + g / 3 + b / 3;

    return average;
}