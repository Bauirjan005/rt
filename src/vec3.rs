use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A 3-dimensional vector with 64-bit floating point precision.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Creates a new `Vec3` from coordinates.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a zero vector `(0, 0, 0)`.
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Creates a vector where all components are equal to `val`.
    pub const fn splat(val: f64) -> Self {
        Self { x: val, y: val, z: val }
    }

    /// Calculates the dot product of two vectors: `self . other`.
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the cross product of two vectors: `self x other`.
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the squared Euclidean length of the vector.
    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    /// Returns the Euclidean length (magnitude) of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the normalized unit vector (magnitude 1.0).
    /// If the vector length is near zero, returns `Vec3::zero()`.
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 1e-12 {
            *self / len
        } else {
            Self::zero()
        }
    }

    /// Returns true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    /// Computes the reflection direction for an incident vector `self` and a surface normal `normal`.
    /// `R = v - 2 * (v . n) * n`
    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * (2.0 * self.dot(normal))
    }

    /// Computes the refraction direction using Snell's Law.
    /// Returns `Some(refracted_vector)` or `None` if total internal reflection occurs.
    pub fn refract(&self, normal: Self, etai_over_etat: f64) -> Option<Self> {
        let cos_theta = (-self.dot(normal)).min(1.0).max(-1.0);
        let r_out_perp = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel_len_sq = 1.0 - r_out_perp.length_squared();

        if r_out_parallel_len_sq < 0.0 {
            None // Total internal reflection
        } else {
            let r_out_parallel = normal * (-r_out_parallel_len_sq.sqrt());
            Some(r_out_perp + r_out_parallel)
        }
    }

    /// Linear interpolation between two vectors.
    pub fn lerp(a: Self, b: Self, t: f64) -> Self {
        a * (1.0 - t) + b * t
    }
}

// Operator Overloads

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

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
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, -5.0, 6.0);
        // 1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert_eq!(v1.dot(v2), 12.0);

        // Perpendicular vectors dot product should be zero
        let px = Vec3::new(1.0, 0.0, 0.0);
        let py = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(px.dot(py), 0.0);
    }

    #[test]
    fn test_cross_product() {
        let px = Vec3::new(1.0, 0.0, 0.0);
        let py = Vec3::new(0.0, 1.0, 0.0);
        let pz = Vec3::new(0.0, 0.0, 1.0);

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

        let zero = Vec3::zero().normalize();
        assert_eq!(zero, Vec3::zero());
    }

    #[test]
    fn test_reflection() {
        let v = Vec3::new(1.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Vec3::new(1.0, 1.0, 0.0));
    }
}
