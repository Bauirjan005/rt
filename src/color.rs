use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Floating-point RGB color with HDR support.
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

    pub const fn new(r: f64, g: f64, b: f64) -> Self { Self { r, g, b } }

    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }

    pub fn scale_brightness(&self, factor: f64) -> Self { *self * factor }

    pub fn to_rgb_clamped(&self) -> (u8, u8, u8) {
        let q = |v: f64| (v * 255.0).round().clamp(0.0, 255.0) as u8;
        (q(self.r), q(self.g), q(self.b))
    }

    pub fn to_ppm_line(&self) -> String {
        let (r, g, b) = self.to_rgb_clamped();
        format!("{r} {g} {b}\n")
    }

    pub fn tint(&self, o: Self) -> Self {
        Self::new(self.r * o.r, self.g * o.g, self.b * o.b)
    }
}

impl Add for Color { type Output = Self; fn add(self, r: Self) -> Self { Self::new(self.r + r.r, self.g + r.g, self.b + r.b) } }
impl Mul<f64> for Color { type Output = Self; fn mul(self, s: f64) -> Self { Self::new(self.r * s, self.g * s, self.b * s) } }
impl Mul<Color> for f64 { type Output = Color; fn mul(self, c: Color) -> Color { c * self } }
impl Mul for Color { type Output = Self; fn mul(self, r: Self) -> Self { self.tint(r) } }
impl AddAssign for Color { fn add_assign(&mut self, r: Self) { *self = *self + r; } }
impl MulAssign<f64> for Color { fn mul_assign(&mut self, s: f64) { *self = *self * s; } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scaling_and_clamping() {
        assert_eq!(Color::new(0.0, 0.5, 1.0).to_rgb_clamped(), (0, 128, 255));
        assert_eq!(Color::new(2.5, 10.0, 1.2).to_rgb_clamped(), (255, 255, 255));
        assert_eq!(Color::new(-0.5, -10.0, 0.0).to_rgb_clamped(), (0, 0, 0));
        assert_eq!(Color::new(0.2, 0.4, 0.6).scale_brightness(2.0).to_rgb_clamped(), (102, 204, 255));
    }

    #[test]
    fn test_ppm_format() {
        assert_eq!(Color::new(1.0, 0.0, 0.5).to_ppm_line(), "255 0 128\n");
    }
}
