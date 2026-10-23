use crate::color::Color;
use crate::vec3::Vec3;

/// Texture pattern applied to object surfaces.
#[derive(Debug, Clone)]
pub enum Texture {
    Solid(Color),
    Checkerboard { color1: Color, color2: Color, scale: f64 },
}

impl Texture {
    pub fn value(&self, _uv: (f64, f64), point: Vec3) -> Color {
        match self {
            Self::Solid(c) => *c,
            Self::Checkerboard { color1, color2, scale } => {
                let sines = (scale * point.x).sin() * (scale * point.y).sin() * (scale * point.z).sin();
                if sines < 0.0 { *color1 } else { *color2 }
            }
        }
    }
}

/// Material properties defining shading, reflection, and refraction.
#[derive(Debug, Clone)]
pub struct Material {
    pub texture: Texture,
    pub specular_exponent: f64,
    pub specular_intensity: f64,
    pub reflectivity: f64,
    pub refractivity: f64,
    pub index_of_refraction: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            texture: Texture::Solid(Color::WHITE),
            specular_exponent: 10.0,
            specular_intensity: 0.1,
            reflectivity: 0.0,
            refractivity: 0.0,
            index_of_refraction: 1.0,
        }
    }
}

impl Material {
    pub fn diffuse(color: Color) -> Self {
        Self { texture: Texture::Solid(color), ..Default::default() }
    }

    pub fn shiny(color: Color, shininess: f64, reflectivity: f64) -> Self {
        Self {
            texture: Texture::Solid(color),
            specular_exponent: shininess,
            specular_intensity: 0.5,
            reflectivity: reflectivity.clamp(0.0, 1.0),
            ..Default::default()
        }
    }

    pub fn checkerboard(c1: Color, c2: Color, scale: f64) -> Self {
        Self {
            texture: Texture::Checkerboard { color1: c1, color2: c2, scale },
            specular_exponent: 20.0,
            specular_intensity: 0.2,
            reflectivity: 0.1,
            ..Default::default()
        }
    }

    pub fn glass(ior: f64) -> Self {
        Self {
            texture: Texture::Solid(Color::new(0.95, 0.95, 0.95)),
            specular_exponent: 125.0,
            specular_intensity: 0.9,
            reflectivity: 0.1,
            refractivity: 0.9,
            index_of_refraction: ior,
        }
    }

    pub fn surface_color(&self, uv: (f64, f64), point: Vec3) -> Color {
        self.texture.value(uv, point)
    }
}
