use glm::GenNum;

use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, Sphere},
    extension::vector_ext::OrthonormalVectorExt
};

use super::{
    SceneObjectError,
    SELFINTERSECTION_TOLERANCE, Cylinder
};

#[derive(Debug)]
enum LensFace {
    Concave(Sphere),
    Convex(Sphere)
}

impl LensFace {
    pub fn center(&self) -> glm::DVec3 {
        match self {
            LensFace::Concave(sp) => sp,
            LensFace::Convex(sp) => sp,
        }.center()
    } 

    pub fn radius(&self) -> f64 {
        match self {
            LensFace::Concave(sp) => sp,
            LensFace::Convex(sp) => sp,
        }.radius()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        match self {
            LensFace::Concave(sp) => sp,
            LensFace::Convex(sp) => sp,
        }.intersect(ray)
    }
}

/// Circular lens object
pub struct Lens {
    axis: Ray,
    thickness: f64,
    radius: f64,
    front: LensFace,
    back: LensFace
}

impl Lens {
    /// Creates a new circular lens object
    /// 
    /// # Arguments
    /// * `axis` - `Ray` with origin at the center of the lens and `direction` pointing towards the front face
    /// * `thickenss` - thickness of the lens
    /// * `radius` - radius of the lens
    /// * `front_radius` - radius of the sphere that makes the front face, negative means concave face
    /// * `back_radius` - radius of the sphere that makes the back face, negative means concave face 
    pub fn new(
        axis: Ray,
        thickness: f64,
        radius: f64,
        front_radius: f64,
        back_radius: f64
    ) -> Result<Self, SceneObjectError> {
        if front_radius.abs() < radius || back_radius.abs() < radius {
            Err(SceneObjectError::LensFacesTooShortError)
        } else if (thickness / 2.) <= SELFINTERSECTION_TOLERANCE { 
            Err(SceneObjectError::LensTooThinError)
        } else {
            let front = Lens::face_construction(*axis.origin(), *axis.direction(), thickness, radius, front_radius);
            let back = Lens::face_construction(*axis.origin(), *axis.direction() * -1., thickness, radius, back_radius);

            let front_pos = glm::distance(*axis.origin(), front.center()) * match front {
                LensFace::Concave(_) => 1.,
                LensFace::Convex(_) => -1.
            };
            let back_pos = -1. * glm::distance(*axis.origin(), back.center()) * match back {
                LensFace::Concave(_) => 1.,
                LensFace::Convex(_) => -1.
            };
            let diff = (front_pos + front_radius) - (back_pos - back_radius);
            
            if diff < SELFINTERSECTION_TOLERANCE {
                Err(SceneObjectError::LensConcaveFaceTooDeepError)
            } else {
                Ok(
                    Self {
                        axis,
                        thickness,
                        radius,
                        front,
                        back
                    }
                )
            }
        }
    }

    fn face_construction(
        position: glm::DVec3,
        normal: glm::DVec3,
        thickness: f64,
        radius: f64,
        face_r: f64
    ) -> LensFace {
        let abs_face_r = face_r.abs();
        let theta = (radius / abs_face_r).asin();
        let tri_height = theta.sin() * abs_face_r;
        let tri_hyposenuse = abs_face_r;
        let tri_length = (tri_hyposenuse.powi(2) - tri_height.powi(2)).sqrt();

        if face_r.is_sign_positive() {
            let face_sphere_center_displ = (thickness / 2.) - tri_length;
            LensFace::Convex(Sphere::new(
                (normal * face_sphere_center_displ) + position,
                abs_face_r
            ))
        } else {
            let face_sphere_center_displ = (thickness / 2.) + tri_length;
            LensFace::Concave(Sphere::new(
                (normal * face_sphere_center_displ) + position,
                abs_face_r
            ))
        }
    }

    fn surface_intersection(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        Cylinder::new(
            self.axis.clone(),
            self.thickness,
            self.radius,
            super::CylinderType::CustomCap
        ).intersect(ray)
            .map(
                |(normal, t)| {
                    if glm::dot(normal, *ray.direction()).is_sign_positive() {
                        (normal * -1., t)
                    } else {
                        (normal, t)
                    }
                }
            )
    }

    fn face_intersection(&self, ray: &Ray, front_face: bool, depth: u64) -> Option<(glm::DVec3, f64)> {
        // TODO understand how it's possible for a ray to intersect a sphere more than once
        if depth >= 2 { return None; }
        let face = if front_face {
            &self.front
        } else {
            &self.back
        };
        face.intersect(ray)
            .and_then(
                |(int_normal, t)| {
                    let hp = *ray.origin() + *ray.direction() * t;
                    let p_a = hp - *self.axis.origin();
                    let dist = glm::length(p_a - *self.axis.direction() * glm::dot(p_a, *self.axis.direction()));
                    if dist < self.radius && glm::distance(*self.axis.origin(), hp) < face.radius() {
                        let normal = match face {
                            LensFace::Convex(_) => int_normal,
                            LensFace::Concave(_) => int_normal * -1.
                        };
                        Some((normal, t))
                    } else {
                        self.face_intersection(
                            &Ray::new(
                                hp,
                                *ray.direction()
                            ),
                            front_face,
                            depth + 1
                        ).map(|(nn, nt)| (nn, nt + t))
                    }
                }
            )
    }
}

impl SceneObjectGeometry for Lens {
    fn intersect(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        let tests = [
            self.surface_intersection(ray),
            if self.thickness.is_finite() { self.face_intersection(ray, true, 0) } else { None },
            if self.thickness.is_finite() { self.face_intersection(ray, false, 0) } else { None }
        ];
        tests.into_iter()
            .flatten()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
    }

    fn bounding_box(&self) -> (glm::DVec3, glm::DVec3) {
        let top_cap_ext = self.front.radius() - glm::distance(*self.axis.origin(), self.front.center());
        let bot_cap_ext = self.back.radius() - glm::distance(*self.axis.direction(), self.back.center());
        let (axis_orth_a, axis_orth_b) = self.axis.direction().orthonormal();
        let (axis_orth_a, axis_orth_b) = (glm::normalize(axis_orth_a), glm::normalize(axis_orth_b));
        let top = *self.axis.origin() + (*self.axis.direction() * ((self.thickness / 2.) + top_cap_ext)).map(|n| if n.is_nan() { 0. } else { n });
        let bottom = *self.axis.origin() - (*self.axis.direction() * ((self.thickness / 2.) + bot_cap_ext)).map(|n| if n.is_nan() { 0. } else { n });
        [
            top + (axis_orth_a * self.radius),
            top - (axis_orth_a * self.radius),
            top + (axis_orth_b * self.radius),
            top - (axis_orth_b * self.radius),
            bottom + (axis_orth_a * self.radius),
            bottom - (axis_orth_a * self.radius),
            bottom + (axis_orth_b * self.radius),
            bottom - (axis_orth_b * self.radius),
        ].into_iter()
            .fold(
                (glm::to_dvec3(std::f64::INFINITY), glm::to_dvec3(std::f64::NEG_INFINITY)),
                |state, cur| {
                    (
                        glm::dvec3(
                            cur.x.min(state.0.x),
                            cur.y.min(state.0.y),
                            cur.z.min(state.0.z)
                        ),
                        glm::dvec3(
                            cur.x.max(state.1.x),
                            cur.y.max(state.1.y),
                            cur.z.max(state.1.z)
                        )
                    )
                }
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinite_radius_faces() {
            let lens = Lens::new(
            Ray::new(glm::to_dvec3(0.), glm::dvec3(1., 0., 0.)),
            1.,
            2.,
            std::f64::INFINITY,
            std::f64::NEG_INFINITY,
        ).unwrap();
        if let LensFace::Convex(sph) = lens.front {
            assert!(sph.radius().is_infinite())
        }
        if let LensFace::Concave(sph) = lens.back {
            assert!(sph.radius().is_infinite())
        }
    }

    #[test]
    fn lens_too_thin() {
        // Lens with both faces being flat
        match Lens::new(
            Ray::new(glm::to_dvec3(0.), glm::dvec3(1., 0., 0.)),
            1.,
            SELFINTERSECTION_TOLERANCE,
            std::f64::NEG_INFINITY,
            std::f64::NEG_INFINITY,
        ) {
            Ok(_) => panic!("Expected error"),
            Err(SceneObjectError::LensTooThinError) => (),
            Err(_) => panic!("Expected `LensTooThinError`")   
        }
    }
}