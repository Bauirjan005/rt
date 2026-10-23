use crate::color::Color;
use crate::light::{AmbientLight, PointLight};
use crate::material::Texture;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;

/// A 3D Scene containing geometric objects, lights, and environment settings.
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<PointLight>,
    pub ambient: AmbientLight,
    pub background_color: Color,
    pub brightness: f64,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            ambient: AmbientLight::default_ambient(),
            background_color: Color::new(0.08, 0.09, 0.12),
            brightness: 1.0,
        }
    }
}

impl Scene {
    pub fn new() -> Self { Self::default() }

    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn add_light(&mut self, light: PointLight) { self.lights.push(light); }
    pub fn set_brightness(&mut self, brightness: f64) { self.brightness = brightness.max(0.0); }
    pub fn set_ambient(&mut self, ambient: AmbientLight) { self.ambient = ambient; }

    pub fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        let mut closest = None;
        for obj in &self.objects {
            if let Some(rec) = obj.hit(ray, t_min, t_max) {
                t_max = rec.t;
                closest = Some(rec);
            }
        }
        closest
    }

    pub fn trace_ray(
        &self,
        ray: &Ray,
        depth: u32,
        max_depth: u32,
        enable_textures: bool,
        enable_reflection: bool,
        enable_refraction: bool,
    ) -> Color {
        if depth >= max_depth {
            return Color::BLACK;
        }

        let rec = match self.hit(ray, 1e-4, f64::INFINITY) {
            Some(r) => r,
            None => return self.background_color * self.brightness,
        };

        let base_color = match (&rec.material.texture, enable_textures) {
            (Texture::Solid(c), _) => *c,
            (Texture::Checkerboard { color1, .. }, false) => *color1,
            _ => rec.material.surface_color(rec.uv, rec.point),
        };

        // Ambient component
        let mut color = base_color * self.ambient.color * (self.ambient.intensity * self.brightness);
        let view_dir = (-ray.direction).normalize();

        // Direct lighting and shadows
        for light in &self.lights {
            let l_vec = light.position - rec.point;
            let l_dist = l_vec.length();
            let l_dir = l_vec.normalize();

            let shadow_ray = Ray::new(rec.point + rec.normal * 1e-4, l_dir);
            if self.hit(&shadow_ray, 1e-4, l_dist - 1e-4).is_none() {
                let n_dot_l = rec.normal.dot(l_dir).max(0.0);
                let diffuse = base_color * light.color * (n_dot_l * light.intensity * self.brightness);

                let halfway = (l_dir + view_dir).normalize();
                let spec = rec.normal.dot(halfway).max(0.0).powf(rec.material.specular_exponent)
                    * rec.material.specular_intensity
                    * light.intensity
                    * self.brightness;

                color += diffuse + light.color * spec;
            }
        }

        // Reflection
        if enable_reflection && rec.material.reflectivity > 0.0 {
            let refl_ray = Ray::new(rec.point + rec.normal * 1e-4, ray.direction.reflect(rec.normal).normalize());
            let refl_col = self.trace_ray(&refl_ray, depth + 1, max_depth, enable_textures, true, enable_refraction);
            let k = rec.material.reflectivity;
            color = color * (1.0 - k) + refl_col * k;
        }

        // Refraction
        if enable_refraction && rec.material.refractivity > 0.0 {
            let eta = if rec.front_face { 1.0 / rec.material.index_of_refraction } else { rec.material.index_of_refraction };
            if let Some(r_dir) = ray.direction.refract(rec.normal, eta) {
                let refr_ray = Ray::new(rec.point - rec.normal * 1e-4, r_dir.normalize());
                let refr_col = self.trace_ray(&refr_ray, depth + 1, max_depth, enable_textures, enable_reflection, true);
                let k = rec.material.refractivity;
                color = color * (1.0 - k) + refr_col * k;
            }
        }

        color
    }
}
