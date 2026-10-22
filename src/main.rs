use rt::renderer::{RenderConfig, Renderer};
use rt::scenarios::get_scenario;
use std::env;
use std::io::{self, stdout, BufWriter};

fn print_help() {
    eprintln!(
        r#"
Ray Tracer (rt) - Usage Guide

USAGE:
    cargo run [OPTIONS] > output.ppm
    cargo run -- [OPTIONS]

OPTIONS:
    -s, --scene <1-4>          Select scenario to render (1: Sphere, 2: Cube/Plane low brightness, 3: All 4 objects, 4: All 4 objects different angle) [default: 3]
    --all, --generate-all      Generate all 4 evaluation scenarios (scene1.ppm, scene2.ppm, scene3.ppm, scene4.ppm) at 800x600
    -w, --width <pixels>       Image width in pixels [default: 800]
    -h, --height <pixels>      Image height in pixels [default: 600]
    -b, --brightness <factor>  Multiply scene brightness by factor [default: 1.0]
    -o, --output <filename>    Save output to specified file path instead of stdout
    -t, --textures             Enable surface textures (bonus feature)
    -r, --reflection           Enable recursive surface reflections (bonus feature)
    --refraction               Enable glass refraction (bonus feature)
    --threads <n>              Number of concurrent rendering threads [default: CPU cores]
    --help                     Show this help message

EXAMPLES:
    cargo run > output.ppm
    cargo run -- --scene 1 > scene1.ppm
    cargo run -- --generate-all
    cargo run -- -s 3 -w 800 -h 600 -t -r -o scene3_bonus.ppm
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut scene_id = 3u32;
    let mut generate_all = false;
    let mut width = 800u32;
    let mut height = 600u32;
    let mut brightness_override: Option<f64> = None;
    let mut output_path: Option<String> = None;
    let mut enable_textures = false;
    let mut enable_reflection = false;
    let mut enable_refraction = false;
    let mut custom_threads: Option<usize> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                print_help();
                return;
            }
            "-s" | "--scene" => {
                if i + 1 < args.len() {
                    scene_id = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Invalid scene ID '{}', defaulting to 3", args[i + 1]);
                        3
                    });
                    i += 1;
                }
            }
            "--all" | "--generate-all" => {
                generate_all = true;
            }
            "-w" | "--width" => {
                if i + 1 < args.len() {
                    width = args[i + 1].parse().unwrap_or(800);
                    i += 1;
                }
            }
            "-h" | "--height" => {
                if i + 1 < args.len() {
                    height = args[i + 1].parse().unwrap_or(600);
                    i += 1;
                }
            }
            "-b" | "--brightness" => {
                if i + 1 < args.len() {
                    brightness_override = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-t" | "--textures" => {
                enable_textures = true;
            }
            "-r" | "--reflection" => {
                enable_reflection = true;
            }
            "--refraction" => {
                enable_refraction = true;
            }
            "--threads" => {
                if i + 1 < args.len() {
                    custom_threads = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            arg => {
                eprintln!("Unknown option: {}", arg);
                print_help();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let mut config = RenderConfig::default();
    config.width = width;
    config.height = height;
    config.enable_textures = enable_textures;
    config.enable_reflection = enable_reflection;
    config.enable_refraction = enable_refraction;
    if let Some(th) = custom_threads {
        config.num_threads = th;
    }

    let renderer = Renderer::new(config.clone());

    if generate_all {
        eprintln!("Generating all 4 required scenarios at 800x600...");
        let aspect_ratio = 800.0 / 600.0;
        let mut all_config = config;
        all_config.width = 800;
        all_config.height = 600;
        let all_renderer = Renderer::new(all_config);

        for id in 1..=4 {
            let filename = format!("scene{}.ppm", id);
            eprintln!("Rendering Scenario {} -> {}...", id, filename);
            let (mut scene, camera) = get_scenario(id, aspect_ratio);
            if let Some(b) = brightness_override {
                scene.set_brightness(b);
            }
            if let Err(e) = all_renderer.render_to_file(&scene, &camera, &filename) {
                eprintln!("Failed to write {}: {}", filename, e);
            } else {
                eprintln!("Successfully rendered {}", filename);
            }
        }
        eprintln!("All 4 scenarios rendered successfully!");
        return;
    }

    let aspect_ratio = (config.width as f64) / (config.height as f64);
    let (mut scene, camera) = get_scenario(scene_id, aspect_ratio);
    if let Some(b) = brightness_override {
        scene.set_brightness(b);
    }

    if let Some(path) = output_path {
        eprintln!(
            "Rendering Scenario {} ({}x{}) to '{}'...",
            scene_id, config.width, config.height, path
        );
        if let Err(e) = renderer.render_to_file(&scene, &camera, &path) {
            eprintln!("Error writing image to {}: {}", path, e);
            std::process::exit(1);
        }
        eprintln!("Done! Image written to {}", path);
    } else {
        // Output to standard output (PPM format)
        let out = stdout();
        let mut writer = BufWriter::new(out.lock());
        if let Err(e) = renderer.render_to_writer(&scene, &camera, &mut writer) {
            if e.kind() != io::ErrorKind::BrokenPipe {
                eprintln!("Error outputting PPM: {}", e);
            }
        }
    }
}
