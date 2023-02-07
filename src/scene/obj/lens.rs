use crate::{
    common::Ray,
    scene::obj::{
        SceneObjectGeometry,
        SceneObjectError,
        Sphere,
        Cylinder,
        SELFINTERSECTION_TOLERANCE
    }
};

#[derive(Debug)]
enum LensFace {
    Concave(Sphere),
    Convex(Sphere)
}

impl LensFace {
    pub fn center(&self) -> &nalgebra_glm::DVec3 {
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

    pub fn intersect(&self, ray: &Ray) -> Option<(nalgebra_glm::DVec3, nalgebra_glm::DVec3, f64)> {
        match self {
            LensFace::Concave(sp) => sp,
            LensFace::Convex(sp) => sp,
        }.intersect(ray)
    }
}

/// Circular lens object
#[derive(Debug)]
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
        } else if thickness <= SELFINTERSECTION_TOLERANCE || (radius / 2.) <= SELFINTERSECTION_TOLERANCE { 
            Err(SceneObjectError::LensTooThinError)
        } else {
            let front = Lens::face_construction(*axis.origin(), *axis.direction(), thickness, radius, front_radius);
            let back = Lens::face_construction(*axis.origin(), axis.direction() * -1., thickness, radius, back_radius);

            let front_pos = axis.origin().metric_distance(front.center()) * match front {
                LensFace::Concave(_) => 1.,
                LensFace::Convex(_) => -1.
            };
            let back_pos = -1. * axis.origin().metric_distance(back.center()) * match back {
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
        position: nalgebra_glm::DVec3,
        normal: nalgebra_glm::DVec3,
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

    fn surface_intersection(&self, ray: &Ray) -> Option<(nalgebra_glm::DVec3, nalgebra_glm::DVec3, f64)> {
        Cylinder::new(
            self.axis.clone(),
            self.thickness,
            self.radius,
            super::CylinderType::CustomCap
        ).intersect(ray)
            .map(
                |(hp, normal, t)| {
                    if normal.dot(ray.direction()).is_sign_positive() {
                        (hp, normal * -1., t)
                    } else {
                        (hp, normal, t)
                    }
                }
            )
    }

    fn face_intersection(
        &self,
        ray: &Ray,
        front_face: bool,
        depth: u64
    ) -> Option<(nalgebra_glm::DVec3, nalgebra_glm::DVec3, f64)> {
        // TODO understand how it's possible for a ray to intersect a sphere more than once
        if depth >= 2 { return None; }
        let face = if front_face {
            &self.front
        } else {
            &self.back
        };
        face.intersect(ray)
            .and_then(
                |(hp, int_normal, t)| {
                    let p_a = hp - self.axis.origin();
                    let dist = (p_a - self.axis.direction() * p_a.dot(self.axis.direction())).magnitude();
                    if dist < self.radius && self.axis.origin().metric_distance(&hp) < face.radius() {
                        let normal = match face {
                            LensFace::Convex(_) => int_normal,
                            LensFace::Concave(_) => int_normal * -1.
                        };
                        Some((hp, normal, t))
                    } else {
                        self.face_intersection(
                            &Ray::new(
                                hp,
                                *ray.direction()
                            ),
                            front_face,
                            depth + 1
                        ).map(|(hp, nn, nt)| (hp, nn, nt + t))
                    }
                }
            )
    }
}

impl SceneObjectGeometry for Lens {
    fn intersect(&self, ray: &Ray) -> Option<(nalgebra_glm::DVec3, nalgebra_glm::DVec3, f64)> {
        let tests = [
            self.surface_intersection(ray),
            if self.thickness.is_finite() { self.face_intersection(ray, true, 0) } else { None },
            if self.thickness.is_finite() { self.face_intersection(ray, false, 0) } else { None }
        ];
        tests.into_iter()
            .flatten()
            .min_by(|(_, _, a), (_, _, b)| a.total_cmp(b))
    }

    fn bounding_box(&self) -> (nalgebra_glm::DVec3, nalgebra_glm::DVec3) {
        let top = self.axis.origin() + self.axis.direction() * (self.thickness / 2.);
        let bottom = self.axis.origin() - self.axis.direction() * (self.thickness / 2.);
        // This creates a really loose bounding box
        let top_sphere = Sphere::new(top, self.radius).bounding_box();
        let bottom_sphere = Sphere::new(bottom, self.radius).bounding_box();
        (
            nalgebra_glm::min2(&top_sphere.0, &bottom_sphere.0),
            nalgebra_glm::max2(&top_sphere.1, &bottom_sphere.1),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinite_radius_faces() {
            let lens = Lens::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.)),
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
        match Lens::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.)),
            1.,
            SELFINTERSECTION_TOLERANCE,
            std::f64::NEG_INFINITY,
            std::f64::NEG_INFINITY,
        ) {
            Ok(_) => panic!("Expected error"),
            Err(SceneObjectError::LensTooThinError) => (),
            Err(_) => panic!("Expected `LensTooThinError`")   
        }
        match Lens::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.)),
            SELFINTERSECTION_TOLERANCE,
            1.,
            std::f64::NEG_INFINITY,
            std::f64::NEG_INFINITY,
        ) {
            Ok(_) => panic!("Expected error"),
            Err(SceneObjectError::LensTooThinError) => (),
            Err(_) => panic!("Expected `LensTooThinError`")   
        }
    }
}