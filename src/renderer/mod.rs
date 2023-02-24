use rayon::prelude::{ParallelBridge, ParallelIterator};

use crate::{tracer::Tracer, camera::Camera, common::Ray, Scene};

pub struct RenderParams {
    pub refraction_index: f64,
    pub samples_per_pixel: u64,
}

#[derive(Debug, Clone)]
pub enum RendererStatus {
    Blank,
    Running,
    Paused,
    Stopped,
    Completed,
    Errored
}

pub struct Renderer {
    width: usize,
    height: usize,
    render_params: RenderParams,
    renderer_status: std::sync::Mutex<RendererStatus>,
    current_sample: std::sync::Mutex<u64>,
    image: std::sync::Mutex<Vec<nalgebra_glm::DVec3>>
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
            renderer_status: std::sync::Mutex::new(RendererStatus::Blank),
            current_sample: std::sync::Mutex::new(0),
            image: std::sync::Mutex::new(
                vec![nalgebra_glm::zero(); width * height]
            )
        }
    }

    pub fn pause(&self) -> Result<(), String> {
        if let Ok(mut rensta) = self.renderer_status.lock() {
            if matches!(*rensta, RendererStatus::Running) {
                *rensta = RendererStatus::Paused;
            }
            Ok(())
        } else {
            Err(String::from("Failed to initialize renderer"))
        }
    }

    pub fn resume(&self) -> Result<(), String> {
        if let Ok(mut rensta) = self.renderer_status.lock() {
            if matches!(*rensta, RendererStatus::Paused) {
                *rensta = RendererStatus::Running;
            }
            Ok(())
        } else {
            Err(String::from("Failed to initialize renderer"))
        }
    }

    pub fn stop(&self) -> Result<(), String> {
        if let Ok(mut rensta) = self.renderer_status.lock() {
            if matches!(*rensta, RendererStatus::Running | RendererStatus::Paused) {
                *rensta = RendererStatus::Stopped;
            }
            Ok(())
        } else {
            Err(String::from("Failed to initialize renderer"))
        }
    }

    fn pass(
        &self,
        tracer: &dyn Tracer,
        camera: &dyn Camera,
        scene: &Scene
    ) -> Vec<nalgebra_glm::DVec3> {
        // Initialy the values in `pass` will be in order of conclusion
        // so map includes the index of the pixel
        let mut pass = (0..(self.width * self.height)).par_bridge()
            .map(
                |i| {
                    let x = i / self.height;
                    let y = i % self.height;
                    let ray = Ray::new(
                        nalgebra_glm::zero(),
                        camera.view_with_filtering(x as f64, y as f64).normalize()
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

    pub fn render(
        &mut self,
        tracer: &dyn Tracer,
        camera: &dyn Camera,
        scene: &Scene
    ) -> Result<(), String> {
        if let Ok(mut rensta) = self.renderer_status.lock() {
            *rensta = RendererStatus::Running;
        } else {
            return Err(String::from("Failed to initialize renderer"))
        }

        loop {
            let rensta = if let Ok(rensta) = self.renderer_status.lock() {
                rensta.clone()
            } else {
                return Err(String::from("Failed to initialize renderer"))
            };

            match rensta {
                RendererStatus::Blank => return Err(String::from("Renderer was blank.")),
                RendererStatus::Running => {
                    // Run a pass over the image
                    let pass = self.pass(tracer, camera, scene);

                    // Incrementing sample count
                    if let Ok(mut sample) = self.current_sample.lock() {
                        // Merging pass into image
                        if let Ok(mut pixels) = self.image.lock() {
                            pixels.iter_mut().zip(pass)
                                .for_each(
                                    |(i, p)| {
                                        *i += p;
                                    }
                                );
                        } else {
                            if let Ok(mut rensta) = self.renderer_status.lock() {
                                *rensta = RendererStatus::Errored;
                            }
                            return Err(String::from("Poisoned image mutex"))
                        }

                        // Increment sample counter
                        *sample += 1;

                        // Conclude render if number of samples is reached
                        if *sample >= self.render_params.samples_per_pixel {
                            break;
                        }
                    } else {
                        if let Ok(mut rensta) = self.renderer_status.lock() {
                            *rensta = RendererStatus::Errored;
                        }
                        return Err(String::from("Poisoned sample mutex"))
                    }
                },
                RendererStatus::Paused => (),
                RendererStatus::Stopped | RendererStatus::Completed | RendererStatus::Errored => break
            }
        }
        if let Ok(mut rensta) = self.renderer_status.lock() {
            *rensta = RendererStatus::Completed;
        }
        Ok(())
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (
            self.width,
            self.height
        )
    }

    pub fn get_image(&self) -> (Vec<nalgebra_glm::DVec3>, u64) {
        (
            self.image.lock().map_or(vec![], |r| r.clone()),
            self.current_sample.lock().map_or(0, |r| r.clone())
        )
    }

    pub fn renderer_status(&self) -> RendererStatus {
        self.renderer_status.lock().map_or(RendererStatus::Errored, |r| r.clone())
    }
}