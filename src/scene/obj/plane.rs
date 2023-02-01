use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, SELFINTERSECTION_TOLERANCE}, extension::vector_ext::OrthonormalVectorExt
};

#[derive(Debug)]
pub struct Plane {
    point: nalgebra_glm::DVec3,
    normal: nalgebra_glm::DVec3
}

impl Plane {
    pub fn new(
        point: nalgebra_glm::DVec3,
        normal: nalgebra_glm::DVec3
    ) -> Self {
        Self {
            point,
            normal
        }
    }

    pub fn point(&self) -> &nalgebra_glm::DVec3 {
        &self.point
    }

    pub fn normal(&self) -> &nalgebra_glm::DVec3 {
        &self.normal
    }
}

impl SceneObjectGeometry for Plane {
    fn intersect(&self, ray: &Ray) -> Option<(nalgebra_glm::DVec3, f64)> {
        let dot = self.normal.dot(ray.direction());
        if !approx::abs_diff_eq!(dot, 0.) {
            let t = (self.normal.dot(&(self.point - ray.origin())))/(dot);
            if t > SELFINTERSECTION_TOLERANCE {
                Some((self.normal, t))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> (nalgebra_glm::DVec3, nalgebra_glm::DVec3) {
        let (orth1, orth2) = self.normal.orthonormal();
        let a = (
            (orth1.abs() + orth2.abs()).map(|n| if approx::abs_diff_eq!(n, 0.) { n } else { n * std::f64::INFINITY})
        ) + self.normal.abs().component_mul(&self.point);
        (
            a * -1.,
            a
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_bb() {
        let p = Plane::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.));
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, nalgebra_glm::DVec3::new(0., f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert_eq!(p_bb.1, nalgebra_glm::DVec3::new(0., f64::INFINITY, f64::INFINITY));

        let p = Plane::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(0., 1., 0.));
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, nalgebra_glm::DVec3::new(f64::NEG_INFINITY, 0., f64::NEG_INFINITY));
        assert_eq!(p_bb.1, nalgebra_glm::DVec3::new(f64::INFINITY, 0., f64::INFINITY));

        let p = Plane::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(0., 0., 1.));
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, nalgebra_glm::DVec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, 0.));
        assert_eq!(p_bb.1, nalgebra_glm::DVec3::new(f64::INFINITY, f64::INFINITY, 0.));

        let v1 = nalgebra_glm::DVec3::new(1. / (2.0_f64).sqrt(), 0., -1. / (2.0_f64).sqrt());
        let p = Plane::new(nalgebra_glm::zero(), v1);
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, nalgebra_glm::DVec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert_eq!(p_bb.1, nalgebra_glm::DVec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY));
    }

    #[test]
    fn front_hit() {
        let p = Plane::new(nalgebra_glm::DVec3::new(5., 0., 0.), nalgebra_glm::DVec3::new(-1., 0., 0.));
        let r = Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.));
        let (normal, t) = p.intersect(&r).expect("Expected intersection");
        approx::assert_abs_diff_eq!(normal, p.normal());
        approx::assert_abs_diff_eq!(t, 5.);
    }

    #[test]
    fn back_hit() {
        let p = Plane::new(nalgebra_glm::DVec3::new(5., 0., 0.), nalgebra_glm::DVec3::new(1., 0., 0.));
        let r = Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.));
        let (normal, t) = p.intersect(&r).expect("Expected intersection");
        approx::assert_abs_diff_eq!(normal, p.normal());
        approx::assert_abs_diff_eq!(t, 5.);
    }

    #[test]
    fn behind_ray_hit() {
        let p = Plane::new(nalgebra_glm::DVec3::new(-5., 0., 0.), nalgebra_glm::DVec3::new(1., 0., 0.));
        let r = Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.));
        assert!(p.intersect(&r).is_none());
    }
}