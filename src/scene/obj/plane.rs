use crate::{
    common::Ray,
    scene::obj::SceneObjectGeometry
};

pub struct Plane {
    normal: glm::DVec3,
    displacement: f64
}

impl Plane {
    pub fn new(
        normal: glm::DVec3,
        displacement: f64
    ) -> Self {
        Self {
            normal,
            displacement
        }
    }
}

impl SceneObjectGeometry for Plane {
    fn intersect(&self, ray: &Ray) -> f64 {
        let d = glm::dot(self.normal, *ray.direction());
        if glm::is_approx_eq(&d, &0.) {
            0.
        } else {
            let t = -1. * (glm::dot(self.normal, *ray.origin()) + self.displacement) / d;
            if t > 1e-6 {
                t
            } else {
                0.
            }
        }
    }

    fn normal(&self, _intersect: &glm::DVec3) -> glm::DVec3 {
        self.normal
    }

    fn bounding_box(&self) -> (glm::DVec3, glm::DVec3) {
        let displ = self.normal * glm::to_dvec3(self.displacement);
        let cross = glm::cross(self.normal, glm::to_dvec3(1.));
        (
            glm::dvec3(
                if glm::is_approx_eq(&cross.x, &0.) {displ.x} else {f64::NEG_INFINITY},
                if glm::is_approx_eq(&cross.y, &0.) {displ.y} else {f64::NEG_INFINITY},
                if glm::is_approx_eq(&cross.z, &0.) {displ.z} else {f64::NEG_INFINITY}
            ),
            glm::dvec3(
                if glm::is_approx_eq(&cross.x, &0.) {displ.x} else {f64::INFINITY},
                if glm::is_approx_eq(&cross.y, &0.) {displ.y} else {f64::INFINITY},
                if glm::is_approx_eq(&cross.z, &0.) {displ.z} else {f64::INFINITY},
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plane_bb() {
        let p = Plane::new(glm::dvec3(1., 0., 0.), 0.);
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(0., f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert_eq!(p_bb.1, glm::dvec3(0., f64::INFINITY, f64::INFINITY));

        let p = Plane::new(glm::dvec3(0., 1., 0.), 0.);
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(f64::NEG_INFINITY, 0., f64::NEG_INFINITY));
        assert_eq!(p_bb.1, glm::dvec3(f64::INFINITY, 0., f64::INFINITY));

        let p = Plane::new(glm::dvec3(0., 0., 1.), 0.);
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(f64::NEG_INFINITY, f64::NEG_INFINITY, 0.));
        assert_eq!(p_bb.1, glm::dvec3(f64::INFINITY, f64::INFINITY, 0.));

        let v1 = glm::dvec3(1. / (2.0_f64).sqrt(), 0., -1. / (2.0_f64).sqrt());
        let p = Plane::new(v1, 0.);
        let p_bb = p.bounding_box();
        assert_eq!(p_bb.0, glm::dvec3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert_eq!(p_bb.1, glm::dvec3(f64::INFINITY, f64::INFINITY, f64::INFINITY));
    }
}