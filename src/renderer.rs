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
        let detected_threads = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Self {
            width: 800,
            height: 600,
            max_depth: 4,
            enable_textures: false,
            enable_reflection: false,
            enable_refraction: false,
            num_threads: detected_threads,
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
        let width = self.config.width;
        let height = self.config.height;
        let num_threads = self.config.num_threads.max(1).min(height as usize);
        let rows_per_thread = (height as usize + num_threads - 1) / num_threads;

        let mut all_pixels = vec![Color::BLACK; (width * height) as usize];

        thread::scope(|s| {
            let mut handles = Vec::with_capacity(num_threads);

            for thread_idx in 0..num_threads {
                let start_y = (thread_idx * rows_per_thread) as u32;
                let end_y = ((thread_idx + 1) * rows_per_thread).min(height as usize) as u32;

                if start_y >= height {
                    break;
                }

                handles.push(s.spawn(move || {
                    let mut thread_pixels = Vec::with_capacity(((end_y - start_y) * width) as usize);
                    for y in start_y..end_y {
                        for x in 0..width {
                            let ray = camera.get_pixel_ray(x, y, width, height);
                            let color = scene.trace_ray(
                                &ray,
                                0,
                                self.config.max_depth,
                                self.config.enable_textures,
                                self.config.enable_reflection,
                                self.config.enable_refraction,
                            );
                            thread_pixels.push(color);
                        }
                    }
                    (start_y, thread_pixels)
                }));
            }

            for handle in handles {
                let (start_y, thread_pixels) = handle.join().expect("Worker thread panicked during render");
                let offset = (start_y * width) as usize;
                all_pixels[offset..offset + thread_pixels.len()].copy_from_slice(&thread_pixels);
            }
        });

        all_pixels
    }

    /// Renders the scene directly into a standard PPM P3 writer.
    pub fn render_to_writer<W: Write>(
        &self,
        scene: &Scene,
        camera: &Camera,
        writer: &mut W,
    ) -> std::io::Result<()> {
        let pixels = self.render_pixels(scene, camera);

        // Write PPM P3 Header
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.config.width, self.config.height)?;
        writeln!(writer, "255")?;

        // Write pixel RGB triplets (one pixel per line)
        for color in pixels {
            let (r, g, b) = color.to_rgb_clamped();
            writeln!(writer, "{} {} {}", r, g, b)?;
        }

        writer.flush()?;
        Ok(())
    }

    /// Renders the scene directly to a file on disk.
    pub fn render_to_file(
        &self,
        scene: &Scene,
        camera: &Camera,
        filepath: &str,
    ) -> std::io::Result<()> {
        let file = File::create(filepath)?;
        let mut writer = BufWriter::new(file);
        self.render_to_writer(scene, camera, &mut writer)
    }

    /// Renders the scene to an in-memory PPM string.
    pub fn render_to_string(&self, scene: &Scene, camera: &Camera) -> String {
        let mut buffer = Vec::new();
        self.render_to_writer(scene, camera, &mut buffer)
            .expect("Writing to in-memory buffer failed");
        String::from_utf8(buffer).expect("Rendered PPM was not valid UTF-8")
    }
}
