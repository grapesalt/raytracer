use crate::math::vec::Vec3;
use image::Rgb;

pub fn hex_rgb(color: u32) -> Rgb<u8> {
    Rgb([
        ((color >> 16) & 0xFF) as u8, // R
        ((color >> 8) & 0xFF) as u8,  // G
        (color & 0xFF) as u8,         // B
    ])
}

pub fn hex_vec(color: u32) -> Vec3 {
    Vec3 {
        x: ((color >> 16) & 0xFF) as f32 / 255.999, // R
        y: ((color >> 8) & 0xFF) as f32 / 255.999,  // G
        z: (color & 0xFF) as f32 / 255.999,         // B
    }
}

impl From<Vec3> for Rgb<u8> {
    fn from(color: Vec3) -> Self {
        Rgb([
            (color.x * 255.999) as u8,
            (color.y * 255.999) as u8,
            (color.z * 255.999) as u8,
        ])
    }
}
