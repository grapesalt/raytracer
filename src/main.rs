use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use math::{ray::Ray, vec::Vec3};
use utils::color;

pub mod math;
pub mod utils;

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

fn hit_sphere(center: Vec3, r: f32, ray: Ray) -> f32 {
    let oc = ray.origin - center;

    // A . A = |A|^2
    let a = ray.direction.magnitude_squared();
    let half_b = Vec3::dot(oc, ray.direction);
    let c = oc.magnitude_squared() - r * r;
    let d = half_b * half_b - a * c; // discriminant

    if d < 0.0 {
        return -1.0;
    } else {
        return (-half_b - f32::sqrt(d)) / a;
    }
}

fn ray_color(r: Ray) -> Rgb<u8> {
    let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        // normal vector
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();

        let color = 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        return Rgb::from(color);
    }

    let unit_direction = r.direction.unit();
    t = 0.5 * (unit_direction.y + 1.0);

    // Lerp between two colors
    let color = (1.0 - t) * color::hex_vec(0xffffff) + t * color::hex_vec(0x80b3ff);
    Rgb::from(color)
}
