use crate::{
    scene::{Scene, obj::SceneObjectMaterial},
    common::{Ray, RandomGen},
    sampler::{Sampler},
    renderer::RenderParams,
    terminator::Terminator,
    extension::vector_ext::OrthonormalVectorExt
};

use super::{Tracer, TracerCapabilities};

/// Simple tracer with Fresnel equation
pub struct FresnelTracer(Box<dyn Terminator>, Box<dyn Sampler>);

impl FresnelTracer {
    pub fn new(terminator: Box<dyn Terminator>, sampler: Box<dyn Sampler>) -> Self {
        Self(
            terminator,
            sampler
        )
    }
}

impl Tracer for FresnelTracer {
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
            let rr_factor = self.0.factor(depth); 

            if let Some(inter) = intersection {
                // Travel the ray to the hit point where the closest object lies and compute the surface normal there.
                let hp = *ray.origin() + *ray.direction() * inter.ray_length();
                let normal = inter.object().normal(&hp);

                let emission_color = glm::to_dvec3(inter.object().emission()) * rr_factor;

                let material_color = match inter.object().material() {
                    SceneObjectMaterial::Diffuse => {
                        let (orth_a, orth_b) = normal.orthonormal();
                        let hemi_sample = glm::normalize(self.1.hemisphere());
                        let rotated = glm::dvec3(
                            glm::dot(glm::dvec3(orth_a.x, orth_b.x, normal.x), hemi_sample),
                            glm::dot(glm::dvec3(orth_a.y, orth_b.y, normal.y), hemi_sample),
                            glm::dot(glm::dvec3(orth_a.z, orth_b.z, normal.z), hemi_sample),
                        );
                        let cost = glm::dot(rotated, normal);
                        let bounce = Ray::new(
                            hp,
                            rotated
                        );
                        let diffuse_color = self.trace(
                            bounce,
                            scene,
                            render_params,
                            depth + 1
                        );
                        ((diffuse_color * *inter.object().color()) * cost) * 0.1 * rr_factor
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
                        ) * rr_factor
                    },
                    SceneObjectMaterial::Refractive => {
                        let refr_ind = render_params.refraction_index;
                        let dot = glm::dot(normal, *ray.direction());
                        let (normal, refr) = if dot > 0. {
                            (normal * -1., render_params.refraction_index)
                        } else {
                            (normal, 1. / render_params.refraction_index)
                        };
                        let cost1 = glm::dot(normal, *ray.direction()) * -1.;
                        let cost2 = 1.0 - refr.powi(2) * (1. - cost1.powi(2));
                        let r0 = ((1. - refr_ind) / (1. + refr_ind)).powi(2);
                        let refr_prob = r0 + (1. - r0) * (1. - cost1).powi(5);
                        let bounce = Ray::new(
                            hp,
                            if cost2 > 0. && RandomGen::rand2() > refr_prob {
                                glm::normalize(*ray.direction() * refr + (normal * (refr * cost1 - cost2.sqrt())))
                            } else {
                                glm::normalize(*ray.direction() + normal * (cost1 * 2.))
                            }
                        );
                        self.trace(
                            bounce,
                            scene,
                            render_params,
                            depth + 1
                        ) * rr_factor
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
            fresnel: true
        }
    }
}