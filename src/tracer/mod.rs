use crate::{common::Ray, scene::Scene, renderer::RenderParams};

mod painterly_tracer;
pub use painterly_tracer::*;

pub trait Tracer: std::marker::Sync {
    fn trace(
        &self,
        ray: Ray,
        scene: &Scene,
        render_params: &RenderParams,
        depth: usize
    ) -> glm::DVec3;
}