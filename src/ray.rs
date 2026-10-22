use crate::vec3::Vec3;

/// A mathematical ray defined by an origin point and a unit direction vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new `Ray` with an origin and direction.
    /// The direction is automatically normalized.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Evaluates the point along the ray at parameter distance `t`.
    /// `P(t) = origin + t * direction`
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(r.at(0.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(r.at(5.0), Vec3::new(1.0, 7.0, 3.0));
    }
}
