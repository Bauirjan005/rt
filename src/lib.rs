pub mod camera;
pub mod color;
pub mod light;
pub mod material;
pub mod objects;
pub mod ray;
pub mod renderer;
pub mod scene;
pub mod scenarios;
pub mod vec3;

// Re-export common types for ergonomics
pub use camera::Camera;
pub use color::Color;
pub use light::{AmbientLight, PointLight};
pub use material::{Material, Texture};
pub use objects::{Cube, Cylinder, HitRecord, Hittable, Plane, Sphere};
pub use ray::Ray;
pub use renderer::{RenderConfig, Renderer};
pub use scene::Scene;
pub use vec3::Vec3;
