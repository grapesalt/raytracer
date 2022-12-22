use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};

pub mod math;

fn main() {
    // Image IO
    let mut img: RgbImage = ImageBuffer::new(256, 256);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = x as u8;
        let g = y as u8;
        let b = 64;

        *pixel = Rgb([r, g, b]);
    }

    img.save_with_format("output/test.png", ImageFormat::Png)
        .unwrap();
}
