use rt::camera::Camera;
use rt::color::Color;
use rt::material::Material;
use rt::objects::{Cube, Cylinder, Hittable, Plane, Sphere};
use rt::ray::Ray;
use rt::vec3::Vec3;

// ============================================================================
// 1. Vector Operations Tests
// ============================================================================

#[test]
fn test_vector_dot_product() {
    let a = Vec3::new(1.0, 3.0, -5.0);
    let b = Vec3::new(4.0, -2.0, -1.0);
    // 1*4 + 3*(-2) + (-5)*(-1) = 4 - 6 + 5 = 3
    assert_eq!(a.dot(b), 3.0);

    // Perpendicular vectors have dot product == 0
    let v_x = Vec3::new(1.0, 0.0, 0.0);
    let v_y = Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(v_x.dot(v_y), 0.0);
}

#[test]
fn test_vector_cross_product() {
    let a = Vec3::new(1.0, 0.0, 0.0);
    let b = Vec3::new(0.0, 1.0, 0.0);
    let c = a.cross(b);
    assert_eq!(c, Vec3::new(0.0, 0.0, 1.0));

    // Anti-commutative property: a x b = - (b x a)
    assert_eq!(b.cross(a), Vec3::new(0.0, 0.0, -1.0));

    // Parallel vectors cross product is zero
    let parallel = a.cross(Vec3::new(2.0, 0.0, 0.0));
    assert_eq!(parallel, Vec3::zero());
}

#[test]
fn test_vector_normalization() {
    let v = Vec3::new(3.0, 0.0, 4.0);
    let norm = v.normalize();
    assert_eq!(norm, Vec3::new(0.6, 0.0, 0.8));
    assert!((norm.length() - 1.0).abs() < 1e-10);

    // Zero vector normalization safely returns zero vector
    let zero = Vec3::zero().normalize();
    assert_eq!(zero, Vec3::zero());
}

// ============================================================================
// 2. Ray-Object Intersections Tests (Sphere, Cube, Plane, Cylinder)
// ============================================================================

#[test]
fn test_sphere_intersections() {
    let sphere = Sphere::new(
        Vec3::new(0.0, 0.0, 10.0),
        2.0,
        Material::diffuse(Color::WHITE),
    );

    // 1. Hit
    let hit_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let hit = sphere.hit(&hit_ray, 0.001, 100.0);
    assert!(hit.is_some());
    let rec = hit.unwrap();
    assert!((rec.t - 8.0).abs() < 1e-4);
    assert_eq!(rec.point, Vec3::new(0.0, 0.0, 8.0));

    // 2. Miss
    let miss_ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    assert!(sphere.hit(&miss_ray, 0.001, 100.0).is_none());

    // 3. Tangent (ray at distance exactly R = 2.0)
    let tangent_ray = Ray::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let tangent_hit = sphere.hit(&tangent_ray, 0.001, 100.0);
    assert!(tangent_hit.is_some());
    assert!((tangent_hit.unwrap().t - 10.0).abs() < 1e-4);
}

#[test]
fn test_plane_intersections() {
    let plane = Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::diffuse(Color::WHITE),
    );

    // 1. Hit from above
    let hit_ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
    let hit = plane.hit(&hit_ray, 0.001, 100.0);
    assert!(hit.is_some());
    let rec = hit.unwrap();
    assert!((rec.t - 5.0).abs() < 1e-4);
    assert_eq!(rec.point, Vec3::new(0.0, 0.0, 0.0));

    // 2. Miss (parallel ray)
    let miss_ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    assert!(plane.hit(&miss_ray, 0.001, 100.0).is_none());

    // 3. Tangent / grazing parallel ray along the plane surface
    let tangent_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    assert!(plane.hit(&tangent_ray, 0.001, 100.0).is_none());
}

#[test]
fn test_cube_intersections() {
    let cube = Cube::cube(
        Vec3::new(0.0, 0.0, 5.0),
        2.0, // x,y in [-1, 1], z in [4, 6]
        Material::diffuse(Color::WHITE),
    );

    // 1. Hit directly in center of front face
    let hit_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let hit = cube.hit(&hit_ray, 0.001, 100.0);
    assert!(hit.is_some());
    let rec = hit.unwrap();
    assert!((rec.t - 4.0).abs() < 1e-4);
    assert_eq!(rec.normal, Vec3::new(0.0, 0.0, -1.0));

    // 2. Miss outside the bounding slab
    let miss_ray = Ray::new(Vec3::new(2.5, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    assert!(cube.hit(&miss_ray, 0.001, 100.0).is_none());

    // 3. Tangent ray grazing the exact edge / face boundary at x = 1.0
    let tangent_ray = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let tangent_hit = cube.hit(&tangent_ray, 0.001, 100.0);
    assert!(tangent_hit.is_some());
    assert!((tangent_hit.unwrap().t - 4.0).abs() < 1e-4);
}

#[test]
fn test_cylinder_intersections() {
    let cylinder = Cylinder::new(
        Vec3::new(0.0, 0.0, 5.0),
        1.0, // radius 1.0
        4.0, // height 4.0 (y in [-2, 2])
        Material::diffuse(Color::WHITE),
    );

    // 1. Hit lateral surface
    let hit_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let hit = cylinder.hit(&hit_ray, 0.001, 100.0);
    assert!(hit.is_some());
    let rec = hit.unwrap();
    assert!((rec.t - 4.0).abs() < 1e-4);
    assert_eq!(rec.normal, Vec3::new(0.0, 0.0, -1.0));

    // 2. Hit top cap from above
    let cap_ray = Ray::new(Vec3::new(0.0, 6.0, 5.0), Vec3::new(0.0, -1.0, 0.0));
    let cap_hit = cylinder.hit(&cap_ray, 0.001, 100.0);
    assert!(cap_hit.is_some());
    let crec = cap_hit.unwrap();
    assert!((crec.t - 4.0).abs() < 1e-4);
    assert_eq!(crec.normal, Vec3::new(0.0, 1.0, 0.0));

    // 3. Miss
    let miss_ray = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    assert!(cylinder.hit(&miss_ray, 0.001, 100.0).is_none());

    // 4. Tangent ray grazing cylinder at x = 1.0
    let tangent_ray = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let tangent_hit = cylinder.hit(&tangent_ray, 0.001, 100.0);
    assert!(tangent_hit.is_some());
    assert!((tangent_hit.unwrap().t - 5.0).abs() < 1e-4);
}

// ============================================================================
// 3. Color Scaling and Clamping Tests
// ============================================================================

#[test]
fn test_color_scaling_and_rgb_bounds() {
    // Normal in-range scaling
    let c = Color::new(0.2, 0.5, 0.8);
    let (r, g, b) = c.to_rgb_clamped();
    assert_eq!(r, 51);
    assert_eq!(g, 128);
    assert_eq!(b, 204);

    // High brightness / HDR overexposure clamp to 255
    let overexposed = Color::new(2.0, 5.5, 100.0);
    let (r_max, g_max, b_max) = overexposed.to_rgb_clamped();
    assert_eq!((r_max, g_max, b_max), (255, 255, 255));

    // Underflow / negative color components clamp to 0
    let underflow = Color::new(-1.0, -0.05, 0.0);
    let (r_min, g_min, b_min) = underflow.to_rgb_clamped();
    assert_eq!((r_min, g_min, b_min), (0, 0, 0));

    // Brightness adjustment scaling
    let dim = Color::new(0.1, 0.2, 0.3);
    let boosted = dim.scale_brightness(3.0);
    let (br, bg, bb) = boosted.to_rgb_clamped();
    assert_eq!(br, 77);
    assert_eq!(bg, 153);
    assert_eq!(bb, 229); // 0.3 * 3.0 = 0.9 * 255.0 = 229.4999... -> rounds to 229
}

// ============================================================================
// 4. Camera Transformations Tests
// ============================================================================

#[test]
fn test_camera_translation_and_rotation_ray_direction() {
    // Initial camera at origin looking forward along -Z
    let mut cam = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        1.0,
    );

    let initial_ray = cam.get_ray(0.5, 0.5);
    assert_eq!(initial_ray.origin, Vec3::new(0.0, 0.0, 0.0));
    assert!((initial_ray.direction.z - (-1.0)).abs() < 1e-6);

    // Translation test: move camera to (10, 20, 30)
    cam.set_position(Vec3::new(10.0, 20.0, 30.0));
    cam.set_target(Vec3::new(10.0, 20.0, 20.0)); // still looking along -Z
    let translated_ray = cam.get_ray(0.5, 0.5);
    assert_eq!(translated_ray.origin, Vec3::new(10.0, 20.0, 30.0));
    assert!((translated_ray.direction.z - (-1.0)).abs() < 1e-6);

    // Rotation test: rotate camera to look towards +X axis
    cam.set_target(Vec3::new(20.0, 20.0, 30.0));
    let rotated_ray = cam.get_ray(0.5, 0.5);
    assert_eq!(rotated_ray.origin, Vec3::new(10.0, 20.0, 30.0));
    assert!((rotated_ray.direction.x - 1.0).abs() < 1e-6);
    assert!(rotated_ray.direction.y.abs() < 1e-6);
    assert!(rotated_ray.direction.z.abs() < 1e-6);

    // Rotation test: rotate camera to look upwards (+Y axis)
    // Note: set vup to (0, 0, 1) when looking along +Y to avoid gimbal lock
    cam.vup = Vec3::new(0.0, 0.0, 1.0);
    cam.set_target(Vec3::new(10.0, 30.0, 30.0));
    let upward_ray = cam.get_ray(0.5, 0.5);
    assert!((upward_ray.direction.y - 1.0).abs() < 1e-6);
}
