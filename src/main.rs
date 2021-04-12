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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Vec3::new(
                (a as f64) + 0.9 * random::<f64>(),
                0.2,
                (b as f64) + 0.9 * random::<f64>(),
            );

            let sphere_material: Rc<Box<dyn Material>> = if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::new_random(0.0, 1.0) * Color::new_random(0.0, 1.0);
                Rc::new(Box::new(Lambertian::new(albedo)))
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::new_random(0.5, 1.0);
                let fuzz = 0.5 * random::<f64>();
                Rc::new(Box::new(Metal::new(albedo, fuzz)))
            } else {
                // grass
                Rc::new(Box::new(Dielectric::new(1.5)))
            };

            world.add(Box::new(Sphere::new(center, 0.2, &sphere_material)));
        }
    }

    let material1: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        &material1,
    )));

    let material2: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        &material2,
    )));

    let material3: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        &material3,
    )));

    world
}

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 1200;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
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
