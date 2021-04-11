use crate::geometry::HitRecord;
use crate::geometry::Ray;
use crate::utils::color::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
