use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// An infinite flat plane defined by a point and a normal vector.
#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            material,
        }
    }

    /// Sets the reference point on the plane.
    pub fn set_point(&mut self, point: Vec3) {
        self.point = point;
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        // If the ray is practically parallel to the plane, there is no hit.
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = (self.point - ray.origin).dot(self.normal) / denom;

        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);

        // Construct planar UV basis
        let u_axis = if self.normal.y.abs() < 0.99 {
            self.normal.cross(Vec3::new(0.0, 1.0, 0.0)).normalize()
        } else {
            self.normal.cross(Vec3::new(1.0, 0.0, 0.0)).normalize()
        };
        let v_axis = self.normal.cross(u_axis).normalize();

        let rel = point - self.point;
        let u = rel.dot(u_axis);
        let v = rel.dot(v_axis);

        Some(HitRecord::new(
            t,
            point,
            self.normal,
            self.material.clone(),
            (u, v),
            ray,
        ))
    }

    fn set_center(&mut self, center: Vec3) {
        self.point = center;
    }

    fn get_center(&self) -> Vec3 {
        self.point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_plane_hit_and_miss_and_parallel() {
        let plane = Plane::new(
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Material::diffuse(Color::WHITE),
        );

        // 1. Direct hit downwards
        let ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = plane.hit(&ray, 0.001, 100.0);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert!((rec.t - 6.0).abs() < 1e-4);
        assert_eq!(rec.point, Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(rec.normal, Vec3::new(0.0, 1.0, 0.0));

        // 2. Parallel ray (miss)
        let parallel_ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert!(plane.hit(&parallel_ray, 0.001, 100.0).is_none());

        // 3. Ray pointing away from plane (behind ray origin)
        let away_ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert!(plane.hit(&away_ray, 0.001, 100.0).is_none());
    }
}
