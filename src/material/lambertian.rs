use super::material::Material;
use crate::geometry::HitRecord;
use crate::geometry::Ray;
use crate::utils::color::Color;
use crate::utils::vec3::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::new_random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(&rec.p, &scatter_direction);
        Some((self.albedo, scattered))
    }
}
