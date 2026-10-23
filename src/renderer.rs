use crate::camera::Camera;
use crate::color::Color;
use crate::scene::Scene;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread;

/// Configuration options for the ray tracing renderer.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub max_depth: u32,
    pub enable_textures: bool,
    pub enable_reflection: bool,
    pub enable_refraction: bool,
    pub num_threads: usize,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            max_depth: 4,
            enable_textures: false,
            enable_reflection: false,
            enable_refraction: false,
            num_threads: thread::available_parallelism().map_or(4, |n| n.get()),
        }
    }
}

pub struct Renderer {
    pub config: RenderConfig,
}

impl Renderer {
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    /// Renders the scene with the specified camera and returns the full 2D pixel grid.
    pub fn render_pixels(&self, scene: &Scene, camera: &Camera) -> Vec<Color> {
        let (w, h) = (self.config.width, self.config.height);
        let n_threads = self.config.num_threads.max(1).min(h as usize);
        let rows_per_chunk = (h as usize + n_threads - 1) / n_threads;
        let chunk_size = rows_per_chunk * w as usize;
        let mut pixels = vec![Color::BLACK; (w * h) as usize];

        thread::scope(|s| {
            for (idx, chunk) in pixels.chunks_mut(chunk_size).enumerate() {
                let start_y = (idx * rows_per_chunk) as u32;
                s.spawn(move || {
                    for (i, px) in chunk.iter_mut().enumerate() {
                        let x = (i % w as usize) as u32;
                        let y = start_y + (i / w as usize) as u32;
                        if y < h {
                            let ray = camera.get_pixel_ray(x, y, w, h);
                            *px = scene.trace_ray(
                                &ray,
                                0,
                                self.config.max_depth,
                                self.config.enable_textures,
                                self.config.enable_reflection,
                                self.config.enable_refraction,
                            );
                        }
                    }
                });
            }
        });

        pixels
    }

    /// Renders the scene directly into a standard PPM P3 writer.
    pub fn render_to_writer<W: Write>(
        &self,
        scene: &Scene,
        camera: &Camera,
        writer: &mut W,
    ) -> std::io::Result<()> {
        writeln!(writer, "P3\n{} {}\n255", self.config.width, self.config.height)?;
        for c in self.render_pixels(scene, camera) {
            let (r, g, b) = c.to_rgb_clamped();
            writeln!(writer, "{r} {g} {b}")?;
        }
        writer.flush()
    }

    /// Renders the scene directly to a file on disk.
    pub fn render_to_file(
        &self,
        scene: &Scene,
        camera: &Camera,
        filepath: &str,
    ) -> std::io::Result<()> {
        self.render_to_writer(scene, camera, &mut BufWriter::new(File::create(filepath)?))
    }

    /// Renders the scene to an in-memory PPM string.
    pub fn render_to_string(&self, scene: &Scene, camera: &Camera) -> String {
        let mut buf = Vec::new();
        self.render_to_writer(scene, camera, &mut buf).expect("render failed");
        String::from_utf8(buf).expect("invalid utf-8")
    }
}
