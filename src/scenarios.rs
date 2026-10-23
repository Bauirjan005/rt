use crate::camera::Camera;
use crate::color::Color;
use crate::light::{AmbientLight, PointLight};
use crate::material::Material;
use crate::objects::{Cube, Cylinder, Plane, Sphere};
use crate::scene::Scene;
use crate::vec3::Vec3;

fn default_cam(from: Vec3, at: Vec3, aspect_ratio: f64) -> Camera {
    Camera::new(from, at, Vec3::new(0.0, 1.0, 0.0), 55.0, aspect_ratio)
}

fn ground_plane(mat: Material) -> Plane {
    Plane::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), mat)
}

/// Returns the (Scene, Camera) pair for one of the 4 project scenarios.
pub fn get_scenario(scenario_id: u32, aspect_ratio: f64) -> (Scene, Camera) {
    match scenario_id {
        1 => scenario_1_sphere(aspect_ratio),
        2 => scenario_2_plane_and_cube_low_brightness(aspect_ratio),
        3 => scenario_3_all_four_objects(aspect_ratio),
        4 => scenario_4_all_four_objects_different_angle(aspect_ratio),
        _ => scenario_3_all_four_objects(aspect_ratio),
    }
}

/// Scenario 1: A scene with a sphere.
pub fn scenario_1_sphere(aspect_ratio: f64) -> (Scene, Camera) {
    let mut scene = Scene::new();
    scene.set_brightness(1.2);
    scene.set_ambient(AmbientLight::new(Color::new(0.3, 0.35, 0.4), 0.25));

    scene.add_object(ground_plane(Material::checkerboard(
        Color::new(0.2, 0.25, 0.3),
        Color::new(0.7, 0.75, 0.8),
        2.0,
    )));
    scene.add_object(Sphere::new(
        Vec3::new(0.0, 0.2, -3.5),
        1.2,
        Material::shiny(Color::new(0.85, 0.25, 0.25), 60.0, 0.35),
    ));

    scene.add_light(PointLight::new(Vec3::new(3.0, 5.0, -1.0), Color::new(1.0, 0.95, 0.9), 1.1));
    scene.add_light(PointLight::new(Vec3::new(-3.0, 3.0, -0.5), Color::new(0.4, 0.6, 0.9), 0.5));

    (scene, default_cam(Vec3::new(0.0, 1.5, 1.5), Vec3::new(0.0, 0.2, -3.5), aspect_ratio))
}

/// Scenario 2: A scene with a flat plane and a cube with lower brightness.
pub fn scenario_2_plane_and_cube_low_brightness(aspect_ratio: f64) -> (Scene, Camera) {
    let mut scene = Scene::new();
    scene.set_brightness(0.45);
    scene.set_ambient(AmbientLight::new(Color::new(0.2, 0.2, 0.25), 0.15));

    scene.add_object(ground_plane(Material::diffuse(Color::new(0.35, 0.4, 0.45))));
    scene.add_object(Cube::cube(
        Vec3::new(0.0, 0.0, -3.5),
        2.0,
        Material::shiny(Color::new(0.2, 0.55, 0.85), 40.0, 0.2),
    ));

    scene.add_light(PointLight::new(Vec3::new(2.5, 4.0, -1.0), Color::new(0.9, 0.9, 0.95), 0.8));

    (scene, default_cam(Vec3::new(1.2, 2.0, 1.8), Vec3::new(0.0, 0.0, -3.5), aspect_ratio))
}

/// Helper constructing the 4-object scene (plane, sphere, cube, cylinder).
pub fn create_all_objects_scene() -> Scene {
    let mut scene = Scene::new();
    scene.set_brightness(1.0);
    scene.set_ambient(AmbientLight::new(Color::new(0.25, 0.28, 0.35), 0.2));

    scene.add_object(ground_plane(Material::checkerboard(
        Color::new(0.25, 0.28, 0.32),
        Color::new(0.65, 0.7, 0.75),
        1.5,
    )));
    scene.add_object(Sphere::new(
        Vec3::new(-1.8, 0.0, -3.5),
        1.0,
        Material::shiny(Color::new(0.9, 0.2, 0.25), 80.0, 0.4),
    ));
    scene.add_object(Cube::cube(
        Vec3::new(1.8, 0.0, -4.0),
        1.8,
        Material::shiny(Color::new(0.2, 0.65, 0.35), 50.0, 0.3),
    ));
    scene.add_object(Cylinder::new(
        Vec3::new(0.0, 0.2, -5.5),
        0.8,
        2.4,
        Material::shiny(Color::new(0.25, 0.45, 0.9), 70.0, 0.35),
    ));

    scene.add_light(PointLight::new(Vec3::new(3.5, 6.0, 0.0), Color::new(1.0, 0.98, 0.92), 1.2));
    scene.add_light(PointLight::new(Vec3::new(-4.0, 4.0, -1.0), Color::new(0.4, 0.5, 0.8), 0.5));

    scene
}

/// Scenario 3: All 4 objects from frontal perspective.
pub fn scenario_3_all_four_objects(aspect_ratio: f64) -> (Scene, Camera) {
    (create_all_objects_scene(), default_cam(Vec3::new(0.0, 2.0, 2.0), Vec3::new(0.0, 0.0, -4.0), aspect_ratio))
}

/// Scenario 4: All 4 objects from alternative camera perspective.
pub fn scenario_4_all_four_objects_different_angle(aspect_ratio: f64) -> (Scene, Camera) {
    (create_all_objects_scene(), default_cam(Vec3::new(-4.5, 3.5, -1.5), Vec3::new(0.0, 0.0, -4.0), aspect_ratio))
}
