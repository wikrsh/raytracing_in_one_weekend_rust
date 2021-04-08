use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    value: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { value: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.value[0]
    }

    pub fn y(&self) -> f64 {
        self.value[1]
    }

    pub fn z(&self) -> f64 {
        self.value[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.value.iter().map(|&v| v * v).sum()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.value
            .iter()
            .zip(other.value.iter())
            .map(|(&v1, &v2)| v1 * v2)
            .sum()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            value: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }

    pub fn unit(&self) -> Self {
        Self { value: self.value } / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut value: [f64; 3] = [0.0; 3];
        for i in 0..3 {
            value[i] = -self.value[i];
        }
        Self { value }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut value = [0.0; 3];
        for i in 0..3 {
            value[i] = self.value[i] + other.value[i];
        }
        Self { value }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.value.iter_mut().enumerate().for_each(|(i, v)| {
            *v += rhs.value[i];
        });
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut value = [0.0; 3];
        for i in 0..3 {
            value[i] = self.value[i] - other.value[i];
        }
        Self { value }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let mut value = [0.0; 3];
        for i in 0..3 {
            value[i] = self.value[i] * rhs;
        }
        Self { value }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.value.iter_mut().for_each(|v| {
            *v *= rhs;
        });
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let mut value = [0.0; 3];
        for i in 0..3 {
            value[i] = self.value[i] / rhs;
        }
        Self { value }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.value.iter_mut().for_each(|v| {
            *v /= rhs;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);

        assert!((v1.dot(&v2) - 6.0).abs() < EPSILON);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v = v1.cross(&v2);

        assert!(v.x().abs() < EPSILON);
        assert!(v.y().abs() < EPSILON);
        assert!((v.z() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn unit() {
        let v = Vec3::new(10.0, 0.0, 0.0);
        let u = v.unit();

        assert!((u.x() - 1.0).abs() < EPSILON);
        assert!(u.y().abs() < EPSILON);
        assert!(u.z().abs() < EPSILON);
    }

    #[test]
    fn neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v_neg = -v1;

        assert!((v_neg.x() - (-1.0)).abs() < EPSILON);
        assert!((v_neg.y() - (-2.0)).abs() < EPSILON);
        assert!((v_neg.z() - (-3.0)).abs() < EPSILON);
    }

    #[test]
    fn add() {
        let v = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(0.5, 1.0, 1.5);

        assert!((v.x() - 1.5).abs() < EPSILON);
        assert!((v.y() - 3.0).abs() < EPSILON);
        assert!((v.z() - 4.5).abs() < EPSILON);
    }

    #[test]
    fn add_assign() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let mut v2 = Vec3::new(1.0, 1.0, 1.0);
        v2 += v1;

        assert!((v2.x() - 2.0).abs() < EPSILON);
        assert!((v2.y() - 3.0).abs() < EPSILON);
        assert!((v2.z() - 4.0).abs() < EPSILON);
    }

    #[test]
    fn sub() {
        let v = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(0.1, 0.2, 0.3);

        assert!((v.x() - 0.9).abs() < EPSILON);
        assert!((v.y() - 1.8).abs() < EPSILON);
        assert!((v.z() - 2.7).abs() < EPSILON);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;

        assert!((v.x() - 2.0).abs() < EPSILON);
        assert!((v.y() - 4.0).abs() < EPSILON);
        assert!((v.z() - 6.0).abs() < EPSILON);
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;

        assert!((v.x() - 1.0).abs() < EPSILON);
        assert!((v.y() - 2.0).abs() < EPSILON);
        assert!((v.z() - 3.0).abs() < EPSILON);
    }
}
