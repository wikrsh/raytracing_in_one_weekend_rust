use crate::geometry::ray::Ray;
use crate::material::Material;
use crate::utils::vec3::Vec3;
use std::rc::Rc;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<Box<dyn Material + 'a>>,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Vec3,
        t: f64,
        r: &Ray,
        outward_normal: &Vec3,
        mat: &Rc<Box<dyn Material + 'a>>,
    ) -> Self {
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
            mat: Rc::clone(mat),
        }
    }
}
