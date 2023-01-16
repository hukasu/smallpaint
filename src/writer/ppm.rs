use super::Writer;
use std::io::Write;
use crate::renderer::Renderer;

pub struct PPMWriter;

impl Writer for PPMWriter {
    fn write(renderer: &Renderer, path: &str) {
        let (width, height) = renderer.get_dimensions();
        let (image, at_sample) = renderer.get_image();
        
        let mut f = std::fs::File::create(path).unwrap();

        writeln!(f, "P3").unwrap();
        writeln!(f, "{} {}", width, height).unwrap();
        writeln!(f, "255").unwrap();
        image.iter()
            .for_each(
                |color| {
                    writeln!(
                        f,
                        "{} {} {}",
                        (color.x / *at_sample as f64).clamp(0., 255.) as u8,
                        (color.y / *at_sample as f64).clamp(0., 255.) as u8,
                        (color.z / *at_sample as f64).clamp(0., 255.) as u8
                    ).unwrap();
                }
            );
    }
}