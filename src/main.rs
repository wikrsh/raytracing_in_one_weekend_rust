use rand::prelude::random;
use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::geometry::{Hittable, HittableList, Ray, Sphere};
use raytracing_in_one_weekend::material::{Dielectric, Lambertian, Material, Metal};
use raytracing_in_one_weekend::utils::color::{write_color, Color};
use raytracing_in_one_weekend::utils::vec3::Vec3;
use std::io::{self, BufWriter};
use std::rc::Rc;

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        return if let Some((attenuation, scattered)) = rec.mat.as_ref().scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        };
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
    let max_depth = 50;

    // World

    let mut world = HittableList::new();

    let material_ground: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))));
    let material_center: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))));
    let material_left: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    let material_right: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0)));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        &material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        &material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    // Camera
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        2.0,
        (lookfrom - lookat).length(),
    );

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

                pixel_color += ray_color(&r, &world, max_depth);
            }

            write_color(&mut writer, &pixel_color, samples_per_pixel)?;
        }
    }

    eprintln!("Done.");

    Ok(())
}
