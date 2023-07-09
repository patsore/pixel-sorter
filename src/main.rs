use pixel_sorter::*;

fn main() {
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

    use std::time::Instant;
    let now = Instant::now();
    sorter.sort();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    sorter.current_image.save("./image-output.jpg").expect("Couldn't save image");
}
