use glm::GenNum;

use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, SELFINTERSECTION_TOLERANCE}, extension::vector_ext::OrthonormalVectorExt
};

#[derive(Debug)]
pub struct Plane {
    point: glm::DVec3,
    normal: glm::DVec3
}

impl Plane {
    pub fn new(
        point: glm::DVec3,
        normal: glm::DVec3
    ) -> Self {
        Self {
            point,
            normal
        }
    }

    pub fn point(&self) -> &glm::DVec3 {
        &self.point
    }

    pub fn normal(&self) -> &glm::DVec3 {
        &self.normal
    }
}

impl SceneObjectGeometry for Plane {
    fn intersect(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        let test = glm::dot(self.normal, *ray.direction());
        if !glm::is_approx_eq(&test, &0.) {
            let t = (glm::dot(self.normal, self.point - *ray.origin()))/(test);
            if t > SELFINTERSECTION_TOLERANCE {
                if test > 0. {
                    Some((self.normal * -1., t))
                } else {
                    Some((self.normal, t))
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> (glm::DVec3, glm::DVec3) {
        let (orth1, orth2) = self.normal.orthonormal();
        let a = (
            (glm::abs(orth1) + glm::abs(orth2)).map(|n| if glm::is_approx_eq(&n, &0.) { n } else { n * std::f64::INFINITY})
        ) + glm::abs(self.normal) * self.point;
        (
            a * -1.,
            a
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glm::is_approx_eq;

    #[test]
    fn plane_bb() {
        let p = Plane::new(glm::dvec3(0., 0., 0.), glm::dvec3(1., 0., 0.));
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(0., f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert_eq!(p_bb.1, glm::dvec3(0., f64::INFINITY, f64::INFINITY));

        let p = Plane::new(glm::dvec3(0., 0., 0.), glm::dvec3(0., 1., 0.));
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(f64::NEG_INFINITY, 0., f64::NEG_INFINITY));
        assert_eq!(p_bb.1, glm::dvec3(f64::INFINITY, 0., f64::INFINITY));

        let p = Plane::new(glm::dvec3(0., 0., 0.), glm::dvec3(0., 0., 1.));
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(f64::NEG_INFINITY, f64::NEG_INFINITY, 0.));
        assert_eq!(p_bb.1, glm::dvec3(f64::INFINITY, f64::INFINITY, 0.));

        let v1 = glm::dvec3(1. / (2.0_f64).sqrt(), 0., -1. / (2.0_f64).sqrt());
        let p = Plane::new(glm::dvec3(0., 0., 0.), v1);
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert_eq!(p_bb.1, glm::dvec3(f64::INFINITY, f64::INFINITY, f64::INFINITY));
    }

    #[test]
    fn front_hit() {
        let p = Plane::new(glm::dvec3(5., 0., 0.), glm::dvec3(-1., 0., 0.));
        let r = Ray::new(glm::to_dvec3(0.), glm::dvec3(1., 0., 0.));
        let (normal, t) = p.intersect(&r).expect("Expected intersection");
        glm::assert_approx_eq!(normal, p.normal);
        glm::assert_approx_eq!(t, 5.);
    }

    #[test]
    fn back_hit() {
        let p = Plane::new(glm::dvec3(5., 0., 0.), glm::dvec3(1., 0., 0.));
        let r = Ray::new(glm::to_dvec3(0.), glm::dvec3(1., 0., 0.));
        let (normal, t) = p.intersect(&r).expect("Expected intersection");
        glm::assert_approx_eq!(normal, p.normal * -1.);
        glm::assert_approx_eq!(t, 5.);
    }

    #[test]
    fn behind_ray_hit() {
        let p = Plane::new(glm::dvec3(-5., 0., 0.), glm::dvec3(1., 0., 0.));
        let r = Ray::new(glm::to_dvec3(0.), glm::dvec3(1., 0., 0.));
        assert!(p.intersect(&r).is_none());
    }
}