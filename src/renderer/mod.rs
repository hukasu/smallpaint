use std::io::Write;
use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::{scene::SceneObjectStorage, tracer::Tracer, camera::Camera, common::Ray, Scene};

pub struct RenderParams {
    pub refraction_index: f64,
    pub samples_per_pixel: u64,
}

pub struct Renderer {
    width: usize,
    height: usize,
    render_params: RenderParams,
    current_sample: std::sync::Mutex<u64>, 
    image: std::sync::Mutex<Vec<glm::DVec3>>
}

impl Renderer {
    pub fn new(
        width: usize,
        height: usize,
        refraction_index: f64,
        samples_per_pixel: u64
    ) -> Self {
        Renderer {
            width,
            height,
            render_params: RenderParams { refraction_index, samples_per_pixel },
            current_sample: std::sync::Mutex::new(0),
            image: std::sync::Mutex::new(
                vec![glm::to_dvec3(0.); width * height]
            )
        }
    }

    fn pass<T, O, C>(&self, tracer: &T, camera: &C, scene: &Scene<O>) -> Vec<glm::DVec3>
    where T: Tracer<O>, O: SceneObjectStorage, C: Camera {
        // Initialy the values in `pass` will be in order of conclusion
        // so map includes the index of the pixel
        let mut pass = (0..(self.width * self.height)).par_bridge()
            .map(
                |i| {
                    let x = i / self.height;
                    let y = i % self.height;
                    let ray = Ray::new(
                        glm::to_dvec3(0.),
                        glm::normalize(camera.view_with_filtering(x as f64, y as f64))
                    );
                    (i, tracer.trace(ray, scene, &self.render_params, 0))
                }
            )
            .collect::<Vec<_>>();
        // Sort by index
        pass.sort_by(|(a, _), (b, _)| a.cmp(b));
        pass.into_iter()
            .map(
                |(_, c)| c
            )
            .collect()
    }

    pub fn render<T, O, C>(&mut self, tracer: &T, camera: &C, scene: &Scene<O>)
    where T: Tracer<O>, O: SceneObjectStorage, C: Camera {
        loop {
            // Pause render if paused by User

            // Stop render if stopped by User

            // Run a pass over the image
            let pass = self.pass(tracer, camera, scene);

            // Incrementing sample count
            if let Ok(mut sample) = self.current_sample.lock() {
                // Merging pass into image
                if let Ok(mut pixels) = self.image.lock() {
                    pixels.iter_mut().zip(pass)
                        .for_each(
                            |(i, p)| {
                                *i = *i + p;
                            }
                        );
                } else {
                    panic!("Poisoned image mutex")
                }
                if *sample % 25 == 0 {
                    self.to_ppm(*sample);
                }

                *sample += 1;
                // Conclude render if number of samples is reached
                if *sample >= self.render_params.samples_per_pixel {
                    break;
                }
            } else {
                panic!("Poisoned sample mutex")
            }
        }
        if let Ok(sample) = self.current_sample.lock() {
            self.to_ppm(*sample);
        }
    }

    #[allow(dead_code)]
    fn to_ppm(&self, at_sample: u64) {
        if let Ok(image) = self.image.lock() {
            std::fs::create_dir_all("./target/output").unwrap();
            let mut f = std::fs::File::create(format!("./target/output/{}.ppm", at_sample)).unwrap();

            writeln!(f, "P3").unwrap();
            writeln!(f, "{} {}", self.width, self.height).unwrap();
            writeln!(f, "255").unwrap();
            image.iter()
                .for_each(
                    |color| {
                        writeln!(
                            f,
                            "{} {} {}",
                            (color.x / at_sample as f64).clamp(0., 255.) as u8,
                            (color.y / at_sample as f64).clamp(0., 255.) as u8,
                            (color.z / at_sample as f64).clamp(0., 255.) as u8
                        ).unwrap();
                    }
                );
        }
    }
}