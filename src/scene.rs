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

impl Scene {
    /// Creates a new empty scene with default lighting.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            ambient: AmbientLight::default_ambient(),
            background_color: Color::new(0.08, 0.09, 0.12), // Sleek dark ambient background
            brightness: 1.0,
        }
    }

    /// Adds a hittable object primitive to the scene.
    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    /// Adds a point light source to the scene.
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    /// Adjusts overall scene brightness factor.
    pub fn set_brightness(&mut self, brightness: f64) {
        self.brightness = brightness.max(0.0);
    }

    /// Sets the ambient light of the scene.
    pub fn set_ambient(&mut self, ambient: AmbientLight) {
        self.ambient = ambient;
    }

    /// Tests intersection of a ray with all objects in the scene, returning the closest hit.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                closest_rec = Some(rec);
            }
        }

        closest_rec
    }

    /// Recursively traces a ray into the scene to calculate pixel color.
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

        let hit = self.hit(ray, 1e-4, f64::INFINITY);
        let rec = match hit {
            Some(r) => r,
            None => return self.background_color * self.brightness,
        };

        // Determine surface base albedo
        let base_color = if enable_textures {
            rec.material.surface_color(rec.uv, rec.point)
        } else {
            match &rec.material.texture {
                Texture::Solid(c) => *c,
                Texture::Checkerboard { color1, .. } => *color1,
            }
        };

        // Ambient illumination component
        let mut final_color = base_color * self.ambient.color * (self.ambient.intensity * self.brightness);

        let view_dir = (-ray.direction).normalize();

        // Direct lighting and shadows from each point light
        for light in &self.lights {
            let light_vec = light.position - rec.point;
            let light_dist = light_vec.length();
            let light_dir = light_vec.normalize();

            // Shadow ray check: offset from surface to avoid self-shadowing acne
            let shadow_ray = Ray::new(rec.point + rec.normal * 1e-4, light_dir);
            let in_shadow = self.hit(&shadow_ray, 1e-4, light_dist - 1e-4).is_some();

            if !in_shadow {
                // Diffuse (Lambertian) reflection
                let n_dot_l = rec.normal.dot(light_dir).max(0.0);
                let diffuse = base_color * light.color * (n_dot_l * light.intensity * self.brightness);

                // Specular (Blinn-Phong) highlight
                let halfway = (light_dir + view_dir).normalize();
                let n_dot_h = rec.normal.dot(halfway).max(0.0);
                let spec_factor = n_dot_h.powf(rec.material.specular_exponent)
                    * rec.material.specular_intensity
                    * light.intensity
                    * self.brightness;
                let specular = light.color * spec_factor;

                final_color += diffuse + specular;
            }
        }

        // Bonus: Specular reflection
        if enable_reflection && rec.material.reflectivity > 0.0 {
            let reflect_dir = ray.direction.reflect(rec.normal).normalize();
            let reflect_ray = Ray::new(rec.point + rec.normal * 1e-4, reflect_dir);
            let reflected_color = self.trace_ray(
                &reflect_ray,
                depth + 1,
                max_depth,
                enable_textures,
                enable_reflection,
                enable_refraction,
            );
            final_color = Color::new(
                final_color.r * (1.0 - rec.material.reflectivity)
                    + reflected_color.r * rec.material.reflectivity,
                final_color.g * (1.0 - rec.material.reflectivity)
                    + reflected_color.g * rec.material.reflectivity,
                final_color.b * (1.0 - rec.material.reflectivity)
                    + reflected_color.b * rec.material.reflectivity,
            );
        }

        // Bonus: Refraction
        if enable_refraction && rec.material.refractivity > 0.0 {
            let refraction_ratio = if rec.front_face {
                1.0 / rec.material.index_of_refraction
            } else {
                rec.material.index_of_refraction
            };

            if let Some(refract_dir) = ray.direction.refract(rec.normal, refraction_ratio) {
                let refract_ray = Ray::new(rec.point - rec.normal * 1e-4, refract_dir.normalize());
                let refracted_color = self.trace_ray(
                    &refract_ray,
                    depth + 1,
                    max_depth,
                    enable_textures,
                    enable_reflection,
                    enable_refraction,
                );
                final_color = Color::new(
                    final_color.r * (1.0 - rec.material.refractivity)
                        + refracted_color.r * rec.material.refractivity,
                    final_color.g * (1.0 - rec.material.refractivity)
                        + refracted_color.g * rec.material.refractivity,
                    final_color.b * (1.0 - rec.material.refractivity)
                        + refracted_color.b * rec.material.refractivity,
                );
            }
        }

        final_color
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
