use rt::renderer::{RenderConfig, Renderer};
use rt::scenarios::get_scenario;
use std::env;
use std::io::{stdout, BufWriter};

fn print_help() {
    eprintln!(
        r#"Ray Tracer (rt) - Usage Guide

USAGE:
    cargo run [OPTIONS] > output.ppm
    cargo run -- [OPTIONS]

OPTIONS:
    -s, --scene <1-4>          Select scenario (1: Sphere, 2: Cube/Plane low brightness, 3: All 4, 4: All 4 alt angle) [default: 3]
    --all, --generate-all      Generate all 4 scenarios (scene1.ppm..scene4.ppm) at 800x600
    -w, --width <pixels>       Image width [default: 800]
    -h, --height <pixels>      Image height [default: 600]
    -b, --brightness <factor>  Multiply scene brightness [default: 1.0]
    -o, --output <filename>    Save output to file instead of stdout
    -t, --textures             Enable textures (bonus)
    -r, --reflection           Enable reflections (bonus)
    --refraction               Enable glass refraction (bonus)
    --threads <n>              Concurrent rendering threads [default: CPU cores]
    --help                     Show this help message"#
    );
}

fn main() {
    let mut scene_id = 3u32;
    let mut generate_all = false;
    let mut width = 800u32;
    let mut height = 600u32;
    let mut brightness: Option<f64> = None;
    let mut output: Option<String> = None;
    let mut threads: Option<usize> = None;
    let mut cfg = RenderConfig::default();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" => return print_help(),
            "-s" | "--scene" => scene_id = args.next().and_then(|v| v.parse().ok()).unwrap_or(3),
            "--all" | "--generate-all" => generate_all = true,
            "-w" | "--width" => width = args.next().and_then(|v| v.parse().ok()).unwrap_or(800),
            "-h" | "--height" => height = args.next().and_then(|v| v.parse().ok()).unwrap_or(600),
            "-b" | "--brightness" => brightness = args.next().and_then(|v| v.parse().ok()),
            "-o" | "--output" => output = args.next(),
            "-t" | "--textures" => cfg.enable_textures = true,
            "-r" | "--reflection" => cfg.enable_reflection = true,
            "--refraction" => cfg.enable_refraction = true,
            "--threads" => threads = args.next().and_then(|v| v.parse().ok()),
            other => {
                eprintln!("Unknown option: {other}");
                print_help();
                std::process::exit(1);
            }
        }
    }

    if generate_all {
        cfg.width = 800;
        cfg.height = 600;
    } else {
        cfg.width = width;
        cfg.height = height;
    }
    if let Some(t) = threads { cfg.num_threads = t; }
    let renderer = Renderer::new(cfg.clone());

    let render_one = |id: u32, out_file: Option<&str>| {
        let aspect = (cfg.width as f64) / (cfg.height as f64);
        let (mut scene, camera) = get_scenario(id, aspect);
        if let Some(b) = brightness { scene.set_brightness(b); }

        if let Some(path) = out_file {
            eprintln!("Rendering Scenario {id} ({}x{}) -> '{path}'...", cfg.width, cfg.height);
            if let Err(e) = renderer.render_to_file(&scene, &camera, path) {
                eprintln!("Error writing {path}: {e}");
                if !generate_all { std::process::exit(1); }
            } else {
                eprintln!("Successfully rendered {path}");
            }
        } else {
            let _ = renderer.render_to_writer(&scene, &camera, &mut BufWriter::new(stdout().lock()));
        }
    };

    if generate_all {
        eprintln!("Generating all 4 required scenarios at 800x600...");
        for id in 1..=4 {
            render_one(id, Some(&format!("scene{id}.ppm")));
        }
        eprintln!("All 4 scenarios rendered successfully!");
    } else {
        render_one(scene_id, output.as_deref());
    }
}
