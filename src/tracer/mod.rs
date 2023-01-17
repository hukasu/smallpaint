use crate::{common::Ray, scene::Scene, renderer::RenderParams};

mod simple_tracer;
pub use simple_tracer::*;

pub struct TracerCapabilities {
    pub caustics: bool,
    pub fresnel: bool,
}

pub trait Tracer: std::marker::Sync {
    fn trace(
        &self,
        ray: Ray,
        scene: &Scene,
        render_params: &RenderParams,
        depth: usize
    ) -> glm::DVec3;

    fn capabilities() -> TracerCapabilities where Self: Sized;
}