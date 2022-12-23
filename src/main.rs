use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use math::{ray::Ray, vec::Vec3};

pub mod math;

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

    for j in (0..img.height()).rev() {
        for i in 0..img.width() {
            let x = i as f32 / (img.width() - 1) as f32;
            let y = j as f32 / (img.height() - 1) as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + x * horizontal + y * vertical - origin,
            );

            let pixel = img.get_pixel_mut(i, j);
            *pixel = ray_color(r);
        }
    }

    img.save_with_format("output/test.png", ImageFormat::Png)
        .unwrap();
}

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

fn hit_sphere(center: Vec3, r: f32, ray: Ray) -> f32 {
    let oc = ray.origin - center;

    let a = Vec3::dot(ray.direction, ray.direction);
    let b = 2.0 * Vec3::dot(oc, ray.direction);
    let c = Vec3::dot(oc, oc) - r * r;
    let d = b * b - 4.0 * a * c; // discriminant

    if d < 0.0 {
        return -1.0;
    } else {
        return (-b - f32::sqrt(d)) / (2.0 * a);
    }
}

fn ray_color(r: Ray) -> Rgb<u8> {
    let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        // normal vector
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();

        let color = 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        return Rgb([
            (color.x * 255.999) as u8,
            (color.y * 255.999) as u8,
            (color.z * 255.999) as u8,
        ]);
    }

    let unit_direction = r.direction.unit();
    t = 0.5 * (unit_direction.y + 1.0);

    // Lerp between two colors
    let color = (1.0 - t) * hex_color_vec(0xffffff) + t * hex_color_vec(0x80b3ff);

    Rgb([
        (color.x * 255.999) as u8,
        (color.y * 255.999) as u8,
        (color.z * 255.999) as u8,
    ])
}
