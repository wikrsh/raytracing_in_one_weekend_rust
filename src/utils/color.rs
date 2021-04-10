use std::io::{self, Write};

use crate::utils::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<T: Write>(
    writer: &mut T,
    pixel_color: &Color,
    samples_per_pixels: i32,
) -> io::Result<()> {
    let scale = 1.0 / samples_per_pixels as f64;
    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    writeln!(writer, "{} {} {}", ir, ig, ib)
}
