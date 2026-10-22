# Ray Tracer (`rt`)

A 3D Ray Tracer built from the ground up in Rust with zero external dependencies.

This ray tracer renders 3D geometric scenes into high-quality 2D images in the standard **Portable PixMap (PPM P3 ASCII)** format, simulating light behavior, shadows, surface materials, and camera perspectives.

---

## Table of Contents
1. [Core Features](#core-features)
2. [Quick Start & Running](#quick-start--running)
3. [The 4 Evaluation Scenarios](#the-4-evaluation-scenarios)
4. [Command-Line Options](#command-line-options)
5. [Code Examples & Guide](#code-examples--guide)
   - [Creating Objects (Sphere, Cube, Plane, Cylinder)](#1-creating-objects)
   - [Adjusting Object Position](#2-adjusting-object-position)
   - [Changing Scene Brightness & Lighting](#3-changing-scene-brightness--lighting)
   - [Moving Camera Position and Angle](#4-moving-camera-position-and-angle)
6. [Bonus Features](#bonus-features)
7. [Unit Testing](#unit-testing)

---

## Core Features

- **Geometric Primitives**: Complete mathematical collision and intersection algorithms for:
  - **Sphere**: Analytical quadratic ray-sphere intersection with spherical UV coordinates.
  - **Cube**: Kay & Kajiya slab method ray-AABB intersection with exact face-normal detection.
  - **Flat Plane**: Infinite planar surface with perpendicular/oblique ray intersection and planar UV basis.
  - **Cylinder**: Capped solid 3D cylinder with lateral body and top/bottom circular end-caps.
- **Lighting & Shadows**:
  - Point lights with adjustable position, color, and intensity.
  - Ambient illumination.
  - Shadow rays calculating occlusion and casting accurate geometric shadows.
  - Blinn-Phong specular highlights.
- **Configurable Camera**: Full 6-DOF camera with eye position (`look_from`), target direction (`look_at`), up-vector (`vup`), field of view (`vfov`), and aspect ratio.
- **Multi-Threaded Rendering**: Parallelized across CPU cores using Rust's `std::thread::scope` for near-instant rendering of 800x600 images without external dependencies.
- **PPM P3 ASCII Output**: Compliant with standard PPM viewers and streaming via `cargo run > output.ppm`.
- **Bonus Capabilities**:
  - Procedural surface textures (`-t` / `--textures`)
  - Recursive specular reflections (`-r` / `--reflection`)
  - Dielectric glass refraction (`--refraction`)

---

## Quick Start & Running

### 1. Default Run (Outputs directly to PPM file)
As specified in the project guidelines, you can pipe standard output directly:
```bash
cargo run > output.ppm
```

### 2. Generate All 4 Evaluation Scenarios (800x600)
To generate the four evaluation PPM files in one command:
```bash
cargo run -- --generate-all
```
This produces:
- `scene1.ppm`: Scene with a sphere (high brightness)
- `scene2.ppm`: Scene with a flat plane and a cube (lower brightness)
- `scene3.ppm`: Scene with one of each of all 4 objects (sphere, cube, cylinder, flat plane)
- `scene4.ppm`: Same scene as #3 rendered from a different camera angle

### 3. Render a Specific Scenario
```bash
cargo run -- --scene 1 -o scene1.ppm
cargo run -- --scene 2 -o scene2.ppm
cargo run -- --scene 3 -o scene3.ppm
cargo run -- --scene 4 -o scene4.ppm
```

### 4. Custom Resolution & Speed Testing
For rapid testing, specify custom dimensions:
```bash
# Fast preview at 400x300:
cargo run -- -s 3 -w 400 -h 300 -o preview.ppm

# Full evaluation resolution 800x600:
cargo run -- -s 3 -w 800 -h 600 -o render.ppm
```

---

## The 4 Evaluation Scenarios

| Scenario | Description | Key Parameters |
| :--- | :--- | :--- |
| **Scenario 1** | Scene with a sphere | Brightness: `1.2`, Sphere centered at `(0.0, 0.2, -3.5)` with radius `1.2` on a patterned floor. |
| **Scenario 2** | Flat plane & cube with lower brightness | Brightness: `0.45` (substantially dimmer), Flat plane + blue cube `2.0` side length. |
| **Scenario 3** | All 4 objects (Sphere, Cube, Cylinder, Plane) | Balanced brightness `1.0`, stage with all 4 geometric primitives casting shadows. Camera at `(0.0, 2.0, 2.0)`. |
| **Scenario 4** | Same as Scenario 3 from a different perspective | Same 4 objects, camera relocated to high left perspective `(-4.5, 3.5, -1.5)`. |

---

## Command-Line Options

| Flag | Argument | Description | Default |
| :--- | :--- | :--- | :--- |
| `-s`, `--scene` | `<1-4>` | Select scene preset to render | `3` |
| `--all`, `--generate-all` | *none* | Render all 4 scenarios to `scene1.ppm` .. `scene4.ppm` | `false` |
| `-w`, `--width` | `<pixels>` | Output width in pixels | `800` |
| `-h`, `--height` | `<pixels>` | Output height in pixels | `600` |
| `-b`, `--brightness` | `<factor>` | Multiplier for scene lighting brightness | `1.0` |
| `-o`, `--output` | `<file>` | Output file path (if omitted, writes to stdout) | `stdout` |
| `-t`, `--textures` | *none* | Enable procedural surface textures | `false` |
| `-r`, `--reflection` | *none* | Enable recursive reflections | `false` |
| `--refraction` | *none* | Enable dielectric refraction | `false` |
| `--threads` | `<n>` | Concurrent rendering threads | CPU cores |
| `--help` | *none* | Print help and options | |

---

## Code Examples & Guide

### 1. Creating Objects

All objects implement the `Hittable` trait and can be instantiated with their geometric parameters and surface `Material`.

```rust
use rt::prelude::*; // or use rt::{Sphere, Cube, Plane, Cylinder, Vec3, Material, Color};

// 1. Create a Sphere: center (x, y, z), radius, material
let sphere = Sphere::new(
    Vec3::new(0.0, 1.0, -3.0), // Center
    1.0,                       // Radius
    Material::shiny(Color::new(0.9, 0.2, 0.2), 60.0, 0.3),
);

// 2. Create a Cube: center (x, y, z), size / side length, material
let cube = Cube::cube(
    Vec3::new(2.0, 0.0, -4.0), // Center
    1.8,                       // Side length (1.8 x 1.8 x 1.8)
    Material::diffuse(Color::new(0.2, 0.7, 0.3)),
);

// 3. Create a Flat Plane: reference point, normal vector, material
let plane = Plane::new(
    Vec3::new(0.0, -1.0, 0.0), // Point on plane
    Vec3::new(0.0, 1.0, 0.0),  // Normal (pointing upward +Y)
    Material::diffuse(Color::new(0.5, 0.5, 0.5)),
);

// 4. Create a Cylinder: center (x, y, z), radius, height, material
let cylinder = Cylinder::new(
    Vec3::new(-2.0, 0.2, -4.0), // Center
    0.8,                        // Radius
    2.4,                        // Height
    Material::shiny(Color::new(0.2, 0.4, 0.9), 70.0, 0.4),
);
```

### 2. Adjusting Object Position

You can reposition any object before rendering using `set_center()` or `set_point()`:

```rust
// Move a sphere to point (1, 1, 1)
let mut my_sphere = Sphere::new(Vec3::zero(), 1.0, Material::diffuse(Color::RED));
my_sphere.set_center(Vec3::new(1.0, 1.0, 1.0));

// Reposition a cube
let mut my_cube = Cube::cube(Vec3::zero(), 2.0, Material::diffuse(Color::BLUE));
my_cube.set_center(Vec3::new(-1.5, 0.0, -3.0));

// Adjust plane level
let mut my_plane = Plane::new(Vec3::zero(), Vec3::new(0.0, 1.0, 0.0), Material::diffuse(Color::WHITE));
my_plane.set_point(Vec3::new(0.0, -2.5, 0.0));
```

### 3. Changing Scene Brightness & Lighting

Brightness is controlled at both the scene level and individual light level:

```rust
let mut scene = Scene::new();

// Set global scene brightness multiplier (e.g. 0.45 for low brightness, 1.5 for high brightness)
scene.set_brightness(0.5);

// Configure ambient light
scene.set_ambient(AmbientLight::new(Color::WHITE, 0.1));

// Add point lights with specific brightness/intensity:
let key_light = PointLight::new(
    Vec3::new(3.0, 6.0, 0.0), // Position
    Color::WHITE,             // Color
    1.2,                      // Intensity / brightness
);
scene.add_light(key_light);
```

### 4. Moving Camera Position and Angle

The camera can be freely positioned and oriented:

```rust
// Create camera: look_from, look_at, vup, vfov_degrees, aspect_ratio
let mut camera = Camera::new(
    Vec3::new(0.0, 2.0, 4.0),   // Eye position (look_from)
    Vec3::new(0.0, 0.0, -3.0),  // Target point (look_at)
    Vec3::new(0.0, 1.0, 0.0),   // Up vector (+Y)
    60.0,                       // Vertical Field of View (FOV)
    800.0 / 600.0,              // Aspect ratio (width / height)
);

// Move camera to a different viewpoint (e.g. high-angle side view):
camera.set_position(Vec3::new(-4.5, 3.5, -1.5));

// Aim camera at a new focus point:
camera.set_target(Vec3::new(1.0, 0.0, -4.0));

// Adjust zoom level / Field of View:
camera.set_fov(45.0);
```

---

## Bonus Features

Bonus features are toggled via command-line flags to preserve baseline performance:

- **Surface Textures (`-t` / `--textures`)**:
  Applies 3D procedural checkerboard patterns onto planes and shapes.
  ```bash
  cargo run -- -s 3 -t -o textured.ppm
  ```

- **Surface Reflections (`-r` / `--reflection`)**:
  Enables recursive ray bouncing for mirror and metallic surfaces.
  ```bash
  cargo run -- -s 3 -r -o reflections.ppm
  ```

- **Refraction (`--refraction`)**:
  Simulates light transmission through glass dielectric materials using Snell's Law and Fresnel equations.
  ```bash
  cargo run -- -s 3 -r --refraction -o full_effects.ppm
  ```

---

## Unit Testing

Execute all unit tests covering vector mathematics, primitive collisions (hit/miss/tangent), color clamping, and camera transformations:

```bash
cargo test
```
