use super::{Tracer, TracerCapabilities};

pub struct FlatTracer;

impl FlatTracer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FlatTracer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tracer for FlatTracer {
    fn trace(
            &self,
            ray: crate::common::Ray,
            scene: &crate::Scene,
            _render_params: &crate::renderer::RenderParams,
            _depth: usize
        ) -> nalgebra_glm::DVec3 {
        let intersection = scene.find_intersection(&ray);
        if let Some(int) = intersection {
            int.object().color() * (8. * int.normal().dot(ray.direction()) * -1.)
        } else {
            nalgebra_glm::DVec3::from_element(0.)
        }
    }

    fn capabilities() -> TracerCapabilities where Self: Sized {
        TracerCapabilities { caustics: false, fresnel: false }
    }
}