use crate::{
    scene::{Scene, obj::SceneObjectMaterial},
    common::Ray,
    sampler::{Sampler},
    renderer::RenderParams,
    terminator::Terminator
};

use super::{Tracer, TracerCapabilities};

pub struct SimpleTracer(Box<dyn Terminator>, Box<dyn Sampler>);

/// Simple tracer
impl SimpleTracer {
    pub fn new(terminator: Box<dyn Terminator>, sampler: Box<dyn Sampler>) -> Self {
        Self(
            terminator,
            sampler
        )
    }
}

impl Tracer for SimpleTracer {
    fn trace(
        &self,
        ray: Ray,
        scene: &Scene,
        render_params: &RenderParams,
        depth: usize
    ) -> nalgebra_glm::DVec3 {
        let zero = nalgebra_glm::zero();
        if self.0.terminate(depth) {
            zero
        } else {
            let intersection = scene.find_intersection(&ray);
            let rr_factor = self.0.factor(depth); 

            if let Some(inter) = intersection {
                // Travel the ray to the hit point where the closest object lies and compute the surface normal there.
                let hp = inter.hit_point();
                let normal = inter.normal();

                let emission_color = nalgebra_glm::DVec3::from_element(inter.object().emission()) * rr_factor;

                let (normal, refr) = {
                    let internal_inter_test = normal.dot(ray.direction());
                    if internal_inter_test > 0. {
                        (normal * -1., render_params.refraction_index)
                    } else {
                        (normal, 1. / render_params.refraction_index)
                    }
                };

                let material_color = match inter.object().material() {
                    SceneObjectMaterial::Diffuse => {
                        let bounce_dir = (normal + self.1.hemisphere()).normalize();
                        let cost = bounce_dir.dot(&normal);
                        let bounce = Ray::new(
                            hp,
                            bounce_dir
                        );
                        let diffuse_color = self.trace(
                            bounce,
                            scene,
                            render_params,
                            depth + 1
                        );
                        ((diffuse_color.component_mul(inter.object().color())) * cost) * 0.1 * rr_factor
                    },
                    SceneObjectMaterial::Specular => {
                        let cost = ray.direction().dot(&normal);
                        let bounce = Ray::new(
                            hp,
                            (ray.direction() - normal * (cost * 2.)).normalize()
                        );
                        self.trace(
                            bounce,
                            scene,
                            render_params,
                            depth + 1
                        ) * rr_factor
                    },
                    SceneObjectMaterial::Refractive => {
                        let cost1 = normal.dot(ray.direction()) * -1.;
                        let cost2 = 1.0 - refr.powi(2) * (1. - cost1.powi(2));
                        if cost2 > 0. {
                            let bounce = Ray::new(
                                hp,
                                (ray.direction() * refr + (normal * (refr * cost1 - cost2.sqrt()))).normalize()
                            );
                            self.trace(
                                bounce,
                                scene,
                                render_params,
                                depth + 1
                            ) * rr_factor
                        } else {
                            zero
                        }
                    }
                };
                emission_color + material_color
            } else {
                zero
            }
        }
    }

    fn capabilities() -> TracerCapabilities {
        TracerCapabilities {
            caustics: true,
            fresnel: false
        }
    }
}