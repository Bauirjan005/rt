use crate::color::Color;
use crate::vec3::Vec3;

/// A point light source illuminating the scene in all directions from a position.
#[derive(Debug, Clone)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(position: Vec3, color: Color, intensity: f64) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_intensity(&mut self, intensity: f64) {
        self.intensity = intensity;
    }
}

/// Ambient light that provides constant background illumination.
#[derive(Debug, Clone)]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f64,
}

impl AmbientLight {
    pub fn new(color: Color, intensity: f64) -> Self {
        Self { color, intensity }
    }

    pub fn default_ambient() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 0.15,
        }
    }
}
