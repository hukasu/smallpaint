use crate::{
    scene::{Scene, obj::SceneObjectMaterial},
    common::Ray,
    sampler::{Sampler},
    renderer::RenderParams,
    terminator::Terminator
};

use super::Tracer;

pub struct PainterlyTracer(Box<dyn Terminator>, Box<dyn Sampler>);

impl PainterlyTracer {
    pub fn new(terminator: Box<dyn Terminator>, sampler: Box<dyn Sampler>) -> Self {
        Self(
            terminator,
            sampler
        )
    }
}

impl Tracer for PainterlyTracer {
    fn trace(
        &self,
        ray: Ray,
        scene: &Scene,
        render_params: &RenderParams,
        depth: usize
    ) -> glm::DVec3 {
        let zero = glm::to_dvec3(0.);
        if self.0.terminate(depth) {
            zero
        } else {
            let intersection = scene.find_intersection(&ray);

            if let Some(inter) = intersection {
                // Travel the ray to the hit point where the closest object lies and compute the surface normal there.
                let hp = *ray.origin() + *ray.direction() * inter.ray_length();
                let normal = inter.object().normal(&hp);

                let emission_color = glm::to_dvec3(inter.object().emission()) * 2.;

                let material_color = match inter.object().material() {
                    SceneObjectMaterial::Diffuse => {
                        let bounce = Ray::new(
                            hp,
                            normal + self.1.hemisphere()
                        );
                        let cost = glm::dot(*bounce.direction(), normal);
                        let diffuse_color = self.trace(
                            bounce,
                            scene,
                            render_params,
                            depth + 1
                        );
                        ((diffuse_color * *inter.object().color()) * cost) * 0.1
                    },
                    SceneObjectMaterial::Specular => {
                        let cost = glm::dot(*ray.direction(), normal);
                        let bounce = Ray::new(
                            hp,
                            glm::normalize(*ray.direction() - normal * (cost * 2.))
                        );
                        self.trace(
                            bounce,
                            scene,
                            render_params,
                            depth + 1
                        )
                    },
                    SceneObjectMaterial::Refractive => {
                        let dot = glm::dot(normal, *ray.direction());
                        let (normal, refr) = if dot > 0. {
                            (normal * -1., render_params.refraction_index)
                        } else {
                            (normal, 1. / render_params.refraction_index)
                        };
                        let cost1 = glm::dot(normal, *ray.direction()) * -1.;
                        let cost2 = 1.0 - refr.powi(2) * (1. - cost1.powi(2));
                        if cost2 > 0. {
                            let bounce = Ray::new(
                                hp,
                                glm::normalize(*ray.direction() * refr + (normal * (refr * cost1 - cost2.sqrt())))
                            );
                            self.trace(
                                bounce,
                                scene,
                                render_params,
                                depth + 1
                            )
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
}