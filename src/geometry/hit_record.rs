use crate::geometry::ray::Ray;
use crate::utils::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}
