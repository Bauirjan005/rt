use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A 3D Sphere defined by a center point and radius.
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable [t_min, t_max] range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        // Compute spherical UV coordinates
        let theta = (-outward_normal.y).clamp(-1.0, 1.0).acos();
        let phi = (-outward_normal.z).atan2(outward_normal.x) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;

        Some(HitRecord::new(
            root,
            point,
            outward_normal,
            self.material.clone(),
            (u, v),
            ray,
        ))
    }

    fn set_center(&mut self, center: Vec3) {
        self.center = center;
    }

    fn get_center(&self) -> Vec3 {
        self.center
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_sphere_hit_and_miss_and_tangent() {
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, 5.0),
            1.0,
            Material::diffuse(Color::WHITE),
        );

        // 1. Direct hit towards center
        let hit_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&hit_ray, 0.001, 100.0);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert!((rec.t - 4.0).abs() < 1e-4);
        assert_eq!(rec.normal, Vec3::new(0.0, 0.0, -1.0));

        // 2. Clear miss
        let miss_ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(sphere.hit(&miss_ray, 0.001, 100.0).is_none());

        // 3. Tangent ray (grazes the sphere at y = 1.0)
        let tangent_ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let tangent_hit = sphere.hit(&tangent_ray, 0.001, 100.0);
        assert!(tangent_hit.is_some());
        let trec = tangent_hit.unwrap();
        assert!((trec.t - 5.0).abs() < 1e-4);
        assert!((trec.point.y - 1.0).abs() < 1e-4);
    }
}
