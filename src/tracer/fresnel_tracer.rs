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
    ) -> nalgebra_glm::DVec3 {
        let zero = nalgebra_glm::zero();
        if self.0.terminate(depth) {
            zero
        } else {
            let intersection = scene.find_intersection(&ray);
            let rr_factor = self.0.factor(depth); 

            if let Some(inter) = intersection {
                // Travel the ray to the hit point where the closest object lies and compute the surface normal there.
                let hp = *ray.origin() + *ray.direction() * inter.ray_length();
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
                        let (orth_a, orth_b) = normal.orthonormal();
                        let hemi_sample = self.1.hemisphere().normalize();
                        let rotated = nalgebra_glm::DVec3::new(
                            nalgebra_glm::DVec3::new(orth_a.x, orth_b.x, normal.x).dot(&hemi_sample),
                            nalgebra_glm::DVec3::new(orth_a.y, orth_b.y, normal.y).dot(&hemi_sample),
                            nalgebra_glm::DVec3::new(orth_a.z, orth_b.z, normal.z).dot(&hemi_sample),
                        );
                        let cost = rotated.dot(&normal);
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
                        let refr_ind = render_params.refraction_index;
                        let cost1 = normal.dot(ray.direction()) * -1.;
                        let cost2 = 1.0 - refr.powi(2) * (1. - cost1.powi(2));
                        let r0 = ((1. - refr_ind) / (1. + refr_ind)).powi(2);
                        let refr_prob = r0 + (1. - r0) * (1. - cost1).powi(5);
                        let bounce = Ray::new(
                            hp,
                            if cost2 > 0. && RandomGen::rand2() > refr_prob {
                                (*ray.direction() * refr + (normal * (refr * cost1 - cost2.sqrt()))).normalize()
                            } else {
                                (*ray.direction() + normal * (cost1 * 2.)).normalize()
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