use crate::ray::Ray;
use crate::vec3::Vec3;

/// A perspective camera that generates primary rays through a viewport into 3D world space.
#[derive(Debug, Clone)]
pub struct Camera {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub vfov: f64, // in degrees
    pub aspect_ratio: f64,

    // Internal computed viewport values
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    /// Creates a new camera and precalculates its viewport basis vectors.
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let mut cam = Self {
            look_from,
            look_at,
            vup,
            vfov,
            aspect_ratio,
            origin: Vec3::zero(),
            lower_left_corner: Vec3::zero(),
            horizontal: Vec3::zero(),
            vertical: Vec3::zero(),
            w: Vec3::zero(),
            u: Vec3::zero(),
            v: Vec3::zero(),
        };
        cam.update_basis();
        cam
    }

    /// Recalculates camera coordinates, basis vectors, and viewport geometry.
    pub fn update_basis(&mut self) {
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        self.w = (self.look_from - self.look_at).normalize();
        self.u = self.vup.cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        self.origin = self.look_from;
        self.horizontal = self.u * viewport_width;
        self.vertical = self.v * viewport_height;
        self.lower_left_corner =
            self.origin - self.horizontal * 0.5 - self.vertical * 0.5 - self.w;
    }

    /// Updates camera position (eye point).
    pub fn set_position(&mut self, look_from: Vec3) {
        self.look_from = look_from;
        self.update_basis();
    }

    /// Updates camera target point (angle of view).
    pub fn set_target(&mut self, look_at: Vec3) {
        self.look_at = look_at;
        self.update_basis();
    }

    /// Updates the field of view in degrees.
    pub fn set_fov(&mut self, vfov: f64) {
        self.vfov = vfov;
        self.update_basis();
    }

    /// Updates the aspect ratio (width / height).
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
        self.update_basis();
    }

    /// Generates a primary ray for normalized viewport coordinates (s, t) in [0.0, 1.0].
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction = self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin;
        Ray::new(self.origin, direction)
    }

    /// Generates a primary ray for pixel coordinates (x, y) with image dimensions (width, height).
    pub fn get_pixel_ray(&self, px: u32, py: u32, width: u32, height: u32) -> Ray {
        let s = (px as f64 + 0.5) / (width as f64);
        let t = ((height - 1 - py) as f64 + 0.5) / (height as f64);
        self.get_ray(s, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_primary_ray_direction() {
        let cam = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        );

        // Center ray (s = 0.5, t = 0.5) should point directly towards (0, 0, -1)
        let center_ray = cam.get_ray(0.5, 0.5);
        assert_eq!(center_ray.origin, Vec3::new(0.0, 0.0, 0.0));
        assert!((center_ray.direction.x).abs() < 1e-6);
        assert!((center_ray.direction.y).abs() < 1e-6);
        assert!((center_ray.direction.z - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_camera_translation() {
        let mut cam = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        );

        // Translate camera to (5.0, 3.0, 10.0)
        cam.set_position(Vec3::new(5.0, 3.0, 10.0));
        cam.set_target(Vec3::new(5.0, 3.0, 0.0));

        let ray = cam.get_ray(0.5, 0.5);
        assert_eq!(ray.origin, Vec3::new(5.0, 3.0, 10.0));
        assert!((ray.direction.z - (-1.0)).abs() < 1e-6);
    }

    #[test]
    fn test_camera_rotation_angle() {
        let mut cam = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        );

        // Rotate to look towards positive X axis (+X)
        cam.set_target(Vec3::new(1.0, 0.0, 0.0));
        let ray = cam.get_ray(0.5, 0.5);
        assert!((ray.direction.x - 1.0).abs() < 1e-6);
        assert!((ray.direction.y).abs() < 1e-6);
        assert!((ray.direction.z).abs() < 1e-6);
    }
}
