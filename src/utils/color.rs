use std::io::{self, Write};

use crate::utils::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<T: Write>(writer: &mut T, color: &Color) -> io::Result<()> {
    let ir = (255.999 * color.x()) as i32;
    let ig = (255.999 * color.y()) as i32;
    let ib = (255.999 * color.z()) as i32;

    writeln!(writer, "{} {} {}", ir, ig, ib)
}
