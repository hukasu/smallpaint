use crate::{common::Ray, scene::Scene, renderer::RenderParams};

mod flat_tracer;
pub use flat_tracer::*;

mod simple_tracer;
pub use simple_tracer::*;

mod fresnel_tracer;
pub use fresnel_tracer::*;

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
    ) -> nalgebra_glm::DVec3;

    fn capabilities() -> TracerCapabilities where Self: Sized;
}