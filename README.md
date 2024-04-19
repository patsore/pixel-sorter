# A blazingly fast 🚀 fast [pixel sorter](https://github.com/kimasendorf/ASDFPixelSort) written in Rust

 With a GUI written with egui, and custom sorting logic behind it, this pixel sorter offers
 - Simple, thresholded sorting
 - Angled sorting (with only a very slight amount of undefined behavior as the angle approaches 90 degrees)
 - Animated sorting (outputs images, bring your own encoder)

All of this is done quite quickly (I haven't done any benchmarks but simple sorting runs in about 15ms on my laptop (Ryzen 5 3500U) for a full-HD image). This makes real-time sorting of video theoretically possible.

Out of what I've seen, this might be the **world's fastest** pixel sorter. Feel free to open an issue if that's not the case, I've only done some surface-level research. I have not tried the After Effects/Premiere Pro plugin.

## Examples 

![sorted birds](https://github.com/patsore/pixel-sorter/assets/80210497/3e2b9118-a115-4ab4-ba05-7e9451b0d1b8)

![sorted image 1](https://github.com/patsore/pixel-sorter/blob/main/image-out-1.jpeg?raw=true)

## Usage

To get the binary, head over to the [releases]() and grab the latest binary. There might be some issues, but most of the basic functionality should work fine.

If you need more custom functionality and can read and write Rust, I'd recommend getting the source and compiling it yourself. I don't think there's anything custom you'd need for this. 


The most janky feature is animation - it works, but changing the amount of frames you'll be interpolating the value over is only possible in the code. 

## Contributing

Contributions are welcome, even if it's fairly small stuff (adding more thresholding \ sorting methods, etc.) 
Bigger contributions (area masking, more fleshed-out animation mode, benchmarks) would be very welcome. I'll probably add it myself when I have more free time.

### P.S

If you have any questions about how to use this - feel free to open an issue. 

See LICENSE if you want to use the code in this repo. 
