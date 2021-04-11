use super::material::Material;
use crate::geometry::HitRecord;
use crate::geometry::Ray;
use crate::utils::color::Color;
use crate::utils::vec3::Vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit().reflect(&rec.normal);
        let scattered = Ray::new(
            &rec.p,
            &(reflected + self.fuzz * Vec3::new_random_in_unit_sphere()),
        );

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
