use crate::ray::Ray;
use crate::vec3::Vec3;

/// Perspective camera generating rays through a 3D viewport.
#[derive(Debug, Clone)]
pub struct Camera {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aspect_ratio: f64,

    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let mut cam = Self {
            look_from, look_at, vup, vfov, aspect_ratio,
            lower_left_corner: Vec3::zero(), horizontal: Vec3::zero(), vertical: Vec3::zero(),
        };
        cam.update_basis();
        cam
    }

    pub fn update_basis(&mut self) {
        let h = (self.vfov.to_radians() * 0.5).tan();
        let (vh, vw) = (2.0 * h, self.aspect_ratio * 2.0 * h);
        let w = (self.look_from - self.look_at).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        self.horizontal = u * vw;
        self.vertical = v * vh;
        self.lower_left_corner = self.look_from - self.horizontal * 0.5 - self.vertical * 0.5 - w;
    }

    pub fn set_position(&mut self, look_from: Vec3) { self.look_from = look_from; self.update_basis(); }
    pub fn set_target(&mut self, look_at: Vec3) { self.look_at = look_at; self.update_basis(); }
    pub fn set_fov(&mut self, vfov: f64) { self.vfov = vfov; self.update_basis(); }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) { self.aspect_ratio = aspect_ratio; self.update_basis(); }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let dir = self.lower_left_corner + self.horizontal * s + self.vertical * t - self.look_from;
        Ray::new(self.look_from, dir)
    }

    pub fn get_pixel_ray(&self, px: u32, py: u32, width: u32, height: u32) -> Ray {
        let s = (px as f64 + 0.5) / width as f64;
        let t = ((height - 1 - py) as f64 + 0.5) / height as f64;
        self.get_ray(s, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_primary_ray_direction() {
        let cam = Camera::new(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, 1.0);
        let r = cam.get_ray(0.5, 0.5);
        assert_eq!(r.origin, Vec3::zero());
        assert!(r.direction.x.abs() < 1e-6 && r.direction.y.abs() < 1e-6 && (r.direction.z - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_camera_translation() {
        let mut cam = Camera::new(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, 1.0);
        cam.set_position(Vec3::new(5.0, 3.0, 10.0));
        cam.set_target(Vec3::new(5.0, 3.0, 0.0));
        let r = cam.get_ray(0.5, 0.5);
        assert_eq!(r.origin, Vec3::new(5.0, 3.0, 10.0));
        assert!((r.direction.z - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_camera_rotation_angle() {
        let mut cam = Camera::new(Vec3::zero(), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, 1.0);
        cam.set_target(Vec3::new(1.0, 0.0, 0.0));
        let r = cam.get_ray(0.5, 0.5);
        assert!((r.direction.x - 1.0).abs() < 1e-6 && r.direction.y.abs() < 1e-6 && r.direction.z.abs() < 1e-6);
    }
}
