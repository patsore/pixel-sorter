use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pixel_sorter::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut sorter = Sorter {
        original_image: Default::default(),
        settings: Settings {
            sort_path_angle: 0.0,
            sort_path: SortingPath::Linear,
            threshold: Threshold {
                min: 0,
                max: 255,
                threshold_type: PixelCharacteristic::Average,
            },
            sort_by: PixelCharacteristic::Average,
        },
        current_image: Default::default(),
    };

    sorter.open_image("./image.jpg");
    c.bench_function("sort", |b| {
        b.iter(|| {
            sorter.sort();
            sorter.reset_current_image();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
