use crate::utils::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.clone() + t * self.direction.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn new() {
        let r = Ray::new(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(4.0, 5.0, 6.0));

        assert!((r.origin.x() - 1.0).abs() < EPSILON);
        assert!((r.origin.y() - 2.0).abs() < EPSILON);
        assert!((r.origin.z() - 3.0).abs() < EPSILON);
        assert!((r.direction.x() - 4.0).abs() < EPSILON);
        assert!((r.direction.y() - 5.0).abs() < EPSILON);
        assert!((r.direction.z() - 6.0).abs() < EPSILON);
    }

    #[test]
    fn at() {
        let r = Ray::new(&Vec3::new(1.0, 0.0, 0.0), &Vec3::new(1.0, 1.0, 1.0));
        let p = r.at(0.5);

        assert!((p.x() - 1.5).abs() < EPSILON);
        assert!((p.y() - 0.5).abs() < EPSILON);
        assert!((p.z() - 0.5).abs() < EPSILON);
    }
}
