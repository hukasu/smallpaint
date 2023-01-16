use crate::renderer::Renderer;

pub mod ppm;

pub trait Writer {
    fn write(renderer: &Renderer, path: &str);
}