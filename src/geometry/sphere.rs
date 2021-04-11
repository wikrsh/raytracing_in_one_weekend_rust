use super::hit_record::HitRecord;
use super::hittable::Hittable;
use super::ray::Ray;
use crate::material::Material;
use crate::utils::vec3::Vec3;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Rc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: &Rc<Box<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            mat: Rc::clone(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin().clone() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, root, r, &outward_normal, &self.mat))
    }
}
