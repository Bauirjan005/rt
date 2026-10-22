use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents an RGB color using floating-point components (typically 0.0 to 1.0, but supports HDR).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Creates a color from 0-255 integer values.
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
        }
    }

    /// Adjusts color brightness by multiplying by a factor.
    pub fn scale_brightness(&self, factor: f64) -> Self {
        *self * factor
    }

    /// Converts floating-point color to 8-bit integer RGB (0-255), clamping out-of-range values.
    pub fn to_rgb_clamped(&self) -> (u8, u8, u8) {
        let r = (self.r * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (self.g * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (self.b * 255.0).round().clamp(0.0, 255.0) as u8;
        (r, g, b)
    }

    /// Formats the pixel color as a standard PPM P3 line: "R G B\n".
    pub fn to_ppm_line(&self) -> String {
        let (r, g, b) = self.to_rgb_clamped();
        format!("{} {} {}\n", r, g, b)
    }

    /// Component-wise product (used for surface albedo reflection).
    pub fn tint(&self, other: Self) -> Self {
        Self::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, color: Color) -> Color {
        color * self
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, scalar: f64) {
        self.r *= scalar;
        self.g *= scalar;
        self.b *= scalar;
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.tint(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scaling_and_clamping() {
        // Test values within standard range [0.0, 1.0]
        let c1 = Color::new(0.0, 0.5, 1.0);
        let (r, g, b) = c1.to_rgb_clamped();
        assert_eq!(r, 0);
        assert_eq!(g, 128);
        assert_eq!(b, 255);

        // Test overexposed values (HDR > 1.0) clamp to 255
        let overexposed = Color::new(2.5, 10.0, 1.2);
        assert_eq!(overexposed.to_rgb_clamped(), (255, 255, 255));

        // Test negative values clamp to 0
        let negative = Color::new(-0.5, -10.0, 0.0);
        assert_eq!(negative.to_rgb_clamped(), (0, 0, 0));

        // Test brightness scaling
        let base = Color::new(0.2, 0.4, 0.6);
        let brighter = base.scale_brightness(2.0);
        let (r2, g2, b2) = brighter.to_rgb_clamped();
        assert_eq!(r2, 102);
        assert_eq!(g2, 204);
        assert_eq!(b2, 255); // 0.6 * 2.0 = 1.2 -> clamped to 255
    }

    #[test]
    fn test_ppm_format() {
        let c = Color::new(1.0, 0.0, 0.5);
        assert_eq!(c.to_ppm_line(), "255 0 128\n");
    }
}
