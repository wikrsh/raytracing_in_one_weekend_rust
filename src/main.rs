use rand::prelude::random;
use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::geometry::hittable::Hittable;
use raytracing_in_one_weekend::geometry::hittable_list::HittableList;
use raytracing_in_one_weekend::geometry::ray::Ray;
use raytracing_in_one_weekend::geometry::sphere::Sphere;
use raytracing_in_one_weekend::utils::color::{write_color, Color};
use raytracing_in_one_weekend::utils::vec3::Vec3;
use std::io::{self, BufWriter};

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut writer = BufWriter::new(io::stdout());

    for h in (0..image_height).rev() {
        eprintln!("Scan lines remaining: {}", h);

        for w in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (w as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (h as f64 + random::<f64>()) / (image_height - 1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(&mut writer, &pixel_color, samples_per_pixel)?;
        }
    }

    eprintln!("Done.");

    Ok(())
}
