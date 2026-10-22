pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

pub use cube::Cube;
pub use cylinder::Cylinder;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Record of an intersection hit between a ray and a scene object.
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Material,
    pub uv: (f64, f64),
}

impl HitRecord {
    /// Creates a new `HitRecord`, orienting the normal vector to always oppose the incoming ray.
    pub fn new(
        t: f64,
        point: Vec3,
        outward_normal: Vec3,
        material: Material,
        uv: (f64, f64),
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            t,
            point,
            normal,
            front_face,
            material,
            uv,
        }
    }
}

/// Trait implemented by all geometric primitives in the ray tracer.
pub trait Hittable: Send + Sync {
    /// Tests for ray intersection in the valid interval `[t_min, t_max]`.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    /// Updates the location/center of the object in 3D world space.
    fn set_center(&mut self, center: Vec3);

    /// Retrieves the current location/center of the object.
    fn get_center(&self) -> Vec3;
}
