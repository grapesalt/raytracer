use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use math::{ray::Ray, vec::Vec3};

pub mod math;

// Color utility functions

fn hex_color_rgb(color: u32) -> Rgb<u8> {
    Rgb([
        ((color >> 16) & 0xFF) as u8, // R
        ((color >> 8) & 0xFF) as u8,  // G
        (color & 0xFF) as u8,         // B
    ])
}

fn hex_color_vec(color: u32) -> Vec3 {
    let rgb = hex_color_rgb(color);

    Vec3 {
        x: rgb[0] as f32 / 255.999,
        y: rgb[1] as f32 / 255.999,
        z: rgb[2] as f32 / 255.999,
    }
}

fn main() {
    // Image IO
    let mut img: RgbImage = ImageBuffer::new(853, 480);

    // Camera

    const VIEWPORT_HEIGHT: f32 = 2.0;
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH as f32, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT as f32, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = Ray::new(
            origin,
            lower_left_corner + x as f32 * horizontal + y as f32 * vertical - origin,
        );

        *pixel = ray_color(r);
    }

    img.save_with_format("output/test.png", ImageFormat::Png)
        .unwrap();
}

fn ray_color(r: Ray) -> Rgb<u8> {
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    // Lerp between two colors
    let color = (1.0 - t) * hex_color_vec(0xf4f4ed) + t * hex_color_vec(0xfc7474);

    Rgb([
        (color.x * 255.999) as u8,
        (color.y * 255.999) as u8,
        (color.z * 255.999) as u8,
    ])
}
