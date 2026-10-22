use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A 3D Cylinder aligned with the Y-axis, defined by center, radius, and height, with optional end-caps.
#[derive(Debug, Clone)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
    pub capped: bool,
    pub material: Material,
}

impl Cylinder {
    /// Creates a capped solid 3D cylinder.
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            height,
            capped: true,
            material,
        }
    }

    /// Creates an open cylinder without end-caps.
    pub fn open(center: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            height,
            capped: false,
            material,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let half_h = self.height * 0.5;
        let y_min = self.center.y - half_h;
        let y_max = self.center.y + half_h;

        let mut closest_t = t_max;
        let mut hit_record: Option<HitRecord> = None;

        // 1. Ray-Cylinder lateral body intersection
        let ox = ray.origin.x - self.center.x;
        let oz = ray.origin.z - self.center.z;
        let dx = ray.direction.x;
        let dz = ray.direction.z;

        let a = dx * dx + dz * dz;
        if a > 1e-9 {
            let half_b = ox * dx + oz * dz;
            let c = ox * ox + oz * oz - self.radius * self.radius;
            let discriminant = half_b * half_b - a * c;

            if discriminant >= 0.0 {
                let sqrtd = discriminant.sqrt();
                for root in [(-half_b - sqrtd) / a, (-half_b + sqrtd) / a] {
                    if root >= t_min && root < closest_t {
                        let y = ray.origin.y + root * ray.direction.y;
                        if y >= y_min && y <= y_max {
                            closest_t = root;
                            let point = ray.at(root);
                            let normal = Vec3::new(
                                (point.x - self.center.x) / self.radius,
                                0.0,
                                (point.z - self.center.z) / self.radius,
                            );

                            let phi = (-normal.z).atan2(normal.x) + std::f64::consts::PI;
                            let u = phi / (2.0 * std::f64::consts::PI);
                            let v = (y - y_min) / self.height;

                            hit_record = Some(HitRecord::new(
                                root,
                                point,
                                normal,
                                self.material.clone(),
                                (u, v),
                                ray,
                            ));
                        }
                    }
                }
            }
        }

        // 2. End-caps intersection (if capped)
        if self.capped && ray.direction.y.abs() > 1e-9 {
            // Top Cap
            let t_top = (y_max - ray.origin.y) / ray.direction.y;
            if t_top >= t_min && t_top < closest_t {
                let pt = ray.at(t_top);
                let dist_sq = (pt.x - self.center.x).powi(2) + (pt.z - self.center.z).powi(2);
                if dist_sq <= self.radius * self.radius {
                    closest_t = t_top;
                    let u = (pt.x - self.center.x + self.radius) / (2.0 * self.radius);
                    let v = (pt.z - self.center.z + self.radius) / (2.0 * self.radius);
                    hit_record = Some(HitRecord::new(
                        t_top,
                        pt,
                        Vec3::new(0.0, 1.0, 0.0),
                        self.material.clone(),
                        (u, v),
                        ray,
                    ));
                }
            }

            // Bottom Cap
            let t_bot = (y_min - ray.origin.y) / ray.direction.y;
            if t_bot >= t_min && t_bot < closest_t {
                let pb = ray.at(t_bot);
                let dist_sq = (pb.x - self.center.x).powi(2) + (pb.z - self.center.z).powi(2);
                if dist_sq <= self.radius * self.radius {
                    let u = (pb.x - self.center.x + self.radius) / (2.0 * self.radius);
                    let v = (pb.z - self.center.z + self.radius) / (2.0 * self.radius);
                    hit_record = Some(HitRecord::new(
                        t_bot,
                        pb,
                        Vec3::new(0.0, -1.0, 0.0),
                        self.material.clone(),
                        (u, v),
                        ray,
                    ));
                }
            }
        }

        hit_record
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
    fn test_cylinder_hit_miss_and_tangent() {
        let cylinder = Cylinder::new(
            Vec3::new(0.0, 0.0, 5.0),
            1.0, // radius 1.0
            2.0, // height 2.0 (y in [-1, 1])
            Material::diffuse(Color::WHITE),
        );

        // 1. Direct hit on body
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = cylinder.hit(&ray, 0.001, 100.0);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert!((rec.t - 4.0).abs() < 1e-4);
        assert_eq!(rec.normal, Vec3::new(0.0, 0.0, -1.0));

        // 2. Direct hit on top cap from above
        let cap_ray = Ray::new(Vec3::new(0.0, 5.0, 5.0), Vec3::new(0.0, -1.0, 0.0));
        let cap_hit = cylinder.hit(&cap_ray, 0.001, 100.0);
        assert!(cap_hit.is_some());
        let crec = cap_hit.unwrap();
        assert!((crec.t - 4.0).abs() < 1e-4); // from y=5 to y=1 is dist 4
        assert_eq!(crec.normal, Vec3::new(0.0, 1.0, 0.0));

        // 3. Clear miss
        let miss_ray = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(cylinder.hit(&miss_ray, 0.001, 100.0).is_none());

        // 4. Tangent ray grazing body at x = 1.0
        let tangent_ray = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let tangent_hit = cylinder.hit(&tangent_ray, 0.001, 100.0);
        assert!(tangent_hit.is_some());
        assert!((tangent_hit.unwrap().t - 5.0).abs() < 1e-4);
    }
}
