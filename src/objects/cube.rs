use crate::material::Material;
use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

/// A 3D Axis-Aligned Cube / Box defined by its center and dimensions.
#[derive(Debug, Clone)]
pub struct Cube {
    pub center: Vec3,
    pub size: Vec3,
    pub material: Material,
}

impl Cube {
    /// Creates a box with specified center and 3D dimensions (width, height, depth).
    pub fn new(center: Vec3, size: Vec3, material: Material) -> Self {
        Self {
            center,
            size,
            material,
        }
    }

    /// Creates a uniform cube with equal width, height, and depth.
    pub fn cube(center: Vec3, side_length: f64, material: Material) -> Self {
        Self::new(center, Vec3::splat(side_length), material)
    }

    /// Creates a box from minimum and maximum corner coordinates.
    pub fn from_min_max(min: Vec3, max: Vec3, material: Material) -> Self {
        let center = (min + max) * 0.5;
        let size = max - min;
        Self::new(center, size, material)
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let half = self.size * 0.5;
        let box_min = self.center - half;
        let box_max = self.center + half;

        let mut t_near = t_min;
        let mut t_far = t_max;

        // Check slabs for X, Y, Z axes
        for axis in 0..3 {
            let (origin_comp, dir_comp, min_comp, max_comp) = match axis {
                0 => (ray.origin.x, ray.direction.x, box_min.x, box_max.x),
                1 => (ray.origin.y, ray.direction.y, box_min.y, box_max.y),
                _ => (ray.origin.z, ray.direction.z, box_min.z, box_max.z),
            };

            if dir_comp.abs() < 1e-9 {
                // Ray is parallel to the slab; check if origin is outside
                if origin_comp < min_comp || origin_comp > max_comp {
                    return None;
                }
            } else {
                let inv_d = 1.0 / dir_comp;
                let mut t0 = (min_comp - origin_comp) * inv_d;
                let mut t1 = (max_comp - origin_comp) * inv_d;
                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                if t0 > t_near {
                    t_near = t0;
                }
                if t1 < t_far {
                    t_far = t1;
                }

                if t_near > t_far {
                    return None; // Missed the box
                }
            }
        }

        let hit_t = if t_near > t_min { t_near } else { t_far };
        if hit_t < t_min || hit_t > t_max {
            return None;
        }

        let point = ray.at(hit_t);
        let rel = point - self.center;

        // Compute normal by determining the closest face
        let eps = 1e-4;
        let nx = if (rel.x.abs() - half.x).abs() < eps {
            rel.x.signum()
        } else {
            0.0
        };
        let ny = if (rel.y.abs() - half.y).abs() < eps {
            rel.y.signum()
        } else {
            0.0
        };
        let nz = if (rel.z.abs() - half.z).abs() < eps {
            rel.z.signum()
        } else {
            0.0
        };

        let normal = if nx != 0.0 || ny != 0.0 || nz != 0.0 {
            Vec3::new(nx, ny, nz).normalize()
        } else {
            // Fallback by component dominance
            let dx = rel.x / half.x;
            let dy = rel.y / half.y;
            let dz = rel.z / half.z;
            let ax = dx.abs();
            let ay = dy.abs();
            let az = dz.abs();
            if ax >= ay && ax >= az {
                Vec3::new(dx.signum(), 0.0, 0.0)
            } else if ay >= ax && ay >= az {
                Vec3::new(0.0, dy.signum(), 0.0)
            } else {
                Vec3::new(0.0, 0.0, dz.signum())
            }
        };

        // UV mapping based on dominant face
        let (u, v) = if normal.x.abs() > 0.5 {
            ((rel.z + half.z) / self.size.z, (rel.y + half.y) / self.size.y)
        } else if normal.y.abs() > 0.5 {
            ((rel.x + half.x) / self.size.x, (rel.z + half.z) / self.size.z)
        } else {
            ((rel.x + half.x) / self.size.x, (rel.y + half.y) / self.size.y)
        };

        Some(HitRecord::new(
            hit_t,
            point,
            normal,
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
    fn test_cube_hit_and_miss_and_tangent() {
        let cube = Cube::cube(
            Vec3::new(0.0, 0.0, 5.0),
            2.0, // bounds: x in [-1, 1], y in [-1, 1], z in [4, 6]
            Material::diffuse(Color::WHITE),
        );

        // 1. Direct hit on the front face (z = 4)
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = cube.hit(&ray, 0.001, 100.0);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert!((rec.t - 4.0).abs() < 1e-4);
        assert_eq!(rec.point, Vec3::new(0.0, 0.0, 4.0));
        assert_eq!(rec.normal, Vec3::new(0.0, 0.0, -1.0));

        // 2. Clear miss
        let miss_ray = Ray::new(Vec3::new(2.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(cube.hit(&miss_ray, 0.001, 100.0).is_none());

        // 3. Tangent ray grazing the side face at x = 1.0
        let tangent_ray = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let tangent_hit = cube.hit(&tangent_ray, 0.001, 100.0);
        // Grazing ray hits along the edge at t = 4.0
        assert!(tangent_hit.is_some());
        assert!((tangent_hit.unwrap().t - 4.0).abs() < 1e-4);
    }
}
