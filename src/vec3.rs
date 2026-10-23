use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 3D vector with 64-bit precision.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    pub const fn zero() -> Self { Self::splat(0.0) }
    pub const fn splat(val: f64) -> Self { Self::new(val, val, val) }

    pub fn dot(&self, o: Self) -> f64 { self.x * o.x + self.y * o.y + self.z * o.z }
    pub fn cross(&self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn length_squared(&self) -> f64 { self.dot(*self) }
    pub fn length(&self) -> f64 { self.length_squared().sqrt() }

    pub fn normalize(&self) -> Self {
        let l = self.length();
        if l > 1e-12 { *self / l } else { Self::zero() }
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs().max(self.y.abs()).max(self.z.abs()) < 1e-8
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * (2.0 * self.dot(normal))
    }

    pub fn refract(&self, normal: Self, etai_over_etat: f64) -> Option<Self> {
        let cos_theta = (-self.dot(normal)).clamp(-1.0, 1.0);
        let r_out_perp = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_par_sq = 1.0 - r_out_perp.length_squared();
        (r_out_par_sq >= 0.0).then(|| r_out_perp - normal * r_out_par_sq.sqrt())
    }

    pub fn lerp(a: Self, b: Self, t: f64) -> Self { a * (1.0 - t) + b * t }
}

// Operator overloads
impl Add for Vec3 { type Output = Self; fn add(self, r: Self) -> Self { Self::new(self.x + r.x, self.y + r.y, self.z + r.z) } }
impl Sub for Vec3 { type Output = Self; fn sub(self, r: Self) -> Self { Self::new(self.x - r.x, self.y - r.y, self.z - r.z) } }
impl Mul<f64> for Vec3 { type Output = Self; fn mul(self, s: f64) -> Self { Self::new(self.x * s, self.y * s, self.z * s) } }
impl Mul<Vec3> for f64 { type Output = Vec3; fn mul(self, v: Vec3) -> Vec3 { v * self } }
impl Mul for Vec3 { type Output = Self; fn mul(self, r: Self) -> Self { Self::new(self.x * r.x, self.y * r.y, self.z * r.z) } }
impl Div<f64> for Vec3 { type Output = Self; fn div(self, s: f64) -> Self { Self::new(self.x / s, self.y / s, self.z / s) } }
impl Neg for Vec3 { type Output = Self; fn neg(self) -> Self { Self::new(-self.x, -self.y, -self.z) } }

impl AddAssign for Vec3 { fn add_assign(&mut self, r: Self) { *self = *self + r; } }
impl SubAssign for Vec3 { fn sub_assign(&mut self, r: Self) { *self = *self - r; } }
impl MulAssign<f64> for Vec3 { fn mul_assign(&mut self, s: f64) { *self = *self * s; } }
impl DivAssign<f64> for Vec3 { fn div_assign(&mut self, s: f64) { *self = *self / s; } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_arithmetic() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v2 - v1, Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(v1 * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(2.0 * v1, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(v2 / 2.0, Vec3::new(2.0, 2.5, 3.0));
        assert_eq!(-v1, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_dot_product() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(4.0, -5.0, 6.0)), 12.0);
        assert_eq!(Vec3::new(1.0, 0.0, 0.0).dot(Vec3::new(0.0, 1.0, 0.0)), 0.0);
    }

    #[test]
    fn test_cross_product() {
        let (px, py, pz) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(px.cross(py), pz);
        assert_eq!(py.cross(pz), px);
        assert_eq!(pz.cross(px), py);
        assert_eq!(py.cross(px), -pz);
    }

    #[test]
    fn test_normalization() {
        let v = Vec3::new(0.0, 3.0, 4.0);
        assert_eq!(v.length(), 5.0);
        let norm = v.normalize();
        assert_eq!(norm, Vec3::new(0.0, 0.6, 0.8));
        assert!((norm.length() - 1.0).abs() < 1e-10);
        assert_eq!(Vec3::zero().normalize(), Vec3::zero());
    }

    #[test]
    fn test_reflection() {
        assert_eq!(
            Vec3::new(1.0, -1.0, 0.0).reflect(Vec3::new(0.0, 1.0, 0.0)),
            Vec3::new(1.0, 1.0, 0.0)
        );
    }
}
