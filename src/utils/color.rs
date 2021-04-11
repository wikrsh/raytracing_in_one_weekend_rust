use super::vec3::Vec3;
use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<T: Write>(
    writer: &mut T,
    pixel_color: &Color,
    samples_per_pixels: i32,
) -> io::Result<()> {
    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixels as f64;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;

    writeln!(writer, "{} {} {}", ir, ig, ib)
}
