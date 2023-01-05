use crate::{common::Ray, scene::{Scene, SceneObjectStorage}, renderer::RenderParams};

mod painterly_tracer;
pub use painterly_tracer::*;

pub trait Tracer<O>: std::marker::Sync
where O: SceneObjectStorage {
    fn trace(
        &self,
        ray: Ray,
        scene: &Scene<O>,
        render_params: &RenderParams,
        depth: usize
    ) -> glm::DVec3;
}