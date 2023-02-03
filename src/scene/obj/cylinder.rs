use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, Plane}, extension::vector_ext::OrthonormalVectorExt
};

use super::SELFINTERSECTION_TOLERANCE;

#[derive(Debug)]
pub enum CylinderType {
    ThroughHole,
    SingleCap,
    DoubleCap,
    CustomCap
}

#[derive(Debug)]
pub struct Cylinder {
    axis: Ray,
    height: f64,
    radius: f64,
    ctype: CylinderType
}

impl Cylinder {
    pub fn new(
        axis: Ray,
        height: f64,
        radius: f64,
        ctype: CylinderType
    ) -> Self {
        Self {
            axis,
            height,
            radius,
            ctype
        }
    }

    fn surface_intersection(&self, ray: &Ray) -> Option<(nalgebra_glm::DVec3, f64)> {
        let rdcd = ray.direction().dot(self.axis.direction());
        let roro = ray.origin().dot(ray.origin());
        let rord = ray.origin().dot(ray.direction());
        let roco = ray.origin().dot(self.axis.origin());
        let rocd = ray.origin().dot(self.axis.direction());
        let rdco = ray.direction().dot(self.axis.origin());
        let coco = self.axis.origin().dot(self.axis.origin());
        let cocd = self.axis.origin().dot(self.axis.direction());

        let a = 1. - (rdcd).powi(2);
        let b = 2. * (rord - rdco + rdcd * (cocd - rocd));
        let c = roro - 2. * roco + coco - (cocd - rocd).powi(2) - self.radius.powi(2);
        let delta = b.powi(2) - 4. * a * c;
        if delta >= 0. {
            [(-b + delta.sqrt()) / (2. * a), (-b - delta.sqrt()) / (2. * a)].into_iter()
                .filter_map(
                    |t| {
                        if t > SELFINTERSECTION_TOLERANCE {
                            let m = rocd + t * rdcd - cocd;
                            if m.abs() < self.height / 2. {
                                let axis_point = self.axis.origin() + self.axis.direction() * m;
                                let hit_point = ray.origin() + ray.direction() * t;
                                Some(((hit_point - axis_point).normalize(), t))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                )
                .min_by(|(_, a), (_, b)| a.total_cmp(b))
        } else {
            None
        }
    }

    fn cap_intersection(&self, ray: &Ray, top_cap: bool) -> Option<(nalgebra_glm::DVec3, f64)> {
        let p = if top_cap {
            Plane::new(self.axis.origin() + (self.axis.direction() * (self.height / 2.)), *self.axis.direction())
        } else {
            Plane::new(self.axis.origin() - (self.axis.direction() * (self.height / 2.)), self.axis.direction() * -1.)
        };
        match (&self.ctype, top_cap) {
            (CylinderType::DoubleCap, _) | (CylinderType::SingleCap, false) => {
                p.intersect(ray)
                    .filter(
                        |int| {
                            let hp = ray.origin() + ray.direction() * int.1;
                            p.point().metric_distance(&hp) < self.radius
                        }
                    )
            },
            _ => None
        }
    }
}

impl SceneObjectGeometry for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<(nalgebra_glm::DVec3, f64)> {
        let tests = [
            self.surface_intersection(ray),
            if self.height.is_finite() { self.cap_intersection(ray, true) } else { None },
            if self.height.is_finite() { self.cap_intersection(ray, false) } else { None }
        ];
        tests.into_iter()
            .flatten()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(
                |(normal, t)| {
                    // Normal always points outward if double capped,
                    // but can point inwards if single capped or through hole
                    // Custom capped cyliders are assumed to having both caps
                    match self.ctype {
                        CylinderType::DoubleCap | CylinderType::CustomCap => (normal, t),
                        _ => {
                            if normal.dot(ray.direction()).is_sign_positive() {
                                (normal * -1., t)
                            } else {
                                (normal, t)
                            }
                        }
                    }
                }
            )
    }

    fn bounding_box(&self) -> (nalgebra_glm::DVec3, nalgebra_glm::DVec3) {
        let (axis_orth_a, axis_orth_b) = self.axis.direction().orthonormal();
        let (axis_orth_a, axis_orth_b) = (axis_orth_a.normalize(), axis_orth_b.normalize());
        let top = self.axis.origin() + (self.axis.direction() * (self.height / 2.)).map(|n| if n.is_nan() { 0. } else { n });
        let bottom = self.axis.origin() - (self.axis.direction() * (self.height / 2.)).map(|n| if n.is_nan() { 0. } else { n });
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
                (nalgebra_glm::DVec3::from_element(std::f64::INFINITY), nalgebra_glm::DVec3::from_element(std::f64::NEG_INFINITY)),
                |state, cur| {
                    (
                        nalgebra_glm::DVec3::new(
                            cur.x.min(state.0.x),
                            cur.y.min(state.0.y),
                            cur.z.min(state.0.z)
                        ),
                        nalgebra_glm::DVec3::new(
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
mod test {
    use super::*;

    #[test]
    fn bounding_box_test() {
        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.)),
            2.,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        approx::assert_abs_diff_eq!(bb.0, nalgebra_glm::DVec3::new(-1.,-1.,-1.));
        approx::assert_abs_diff_eq!(bb.1, nalgebra_glm::DVec3::new(1.,1.,1.));

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(0., 1., 0.)),
            2.,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        approx::assert_abs_diff_eq!(bb.0, nalgebra_glm::DVec3::new(-1.,-1.,-1.));
        approx::assert_abs_diff_eq!(bb.1, nalgebra_glm::DVec3::new(1.,1.,1.));

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(0., 0., 1.)),
            2.,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        approx::assert_abs_diff_eq!(bb.0, nalgebra_glm::DVec3::new(-1.,-1.,-1.));
        approx::assert_abs_diff_eq!(bb.1, nalgebra_glm::DVec3::new(1.,1.,1.));

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.)),
            200.,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        approx::assert_abs_diff_eq!(bb.0, nalgebra_glm::DVec3::new(-100.,-1.,-1.));
        approx::assert_abs_diff_eq!(bb.1, nalgebra_glm::DVec3::new(100.,1.,1.));

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 0.)),
            std::f64::INFINITY,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        assert!(bb.0.x.is_infinite() && bb.0.x.is_sign_negative());
        assert!(bb.1.x.is_infinite() && bb.1.x.is_sign_positive());
        approx::assert_abs_diff_eq!(bb.0.yz(), nalgebra_glm::DVec3::new(-100.,-1.,-1.).yz());
        approx::assert_abs_diff_eq!(bb.1.yz(), nalgebra_glm::DVec3::new(100.,1.,1.).yz());

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 1., 0.).normalize()),
            std::f64::INFINITY,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        assert!(bb.0.x.is_infinite() && bb.0.x.is_sign_negative());
        assert!(bb.1.x.is_infinite() && bb.1.x.is_sign_positive());
        assert!(bb.0.y.is_infinite() && bb.0.y.is_sign_negative());
        assert!(bb.1.y.is_infinite() && bb.1.y.is_sign_positive());
        approx::assert_abs_diff_eq!(bb.0.z, -1.);
        approx::assert_abs_diff_eq!(bb.1.z, 1.);

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 1., 1.).normalize()),
            std::f64::INFINITY,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        assert!(bb.0.x.is_infinite() && bb.0.x.is_sign_negative());
        assert!(bb.1.x.is_infinite() && bb.1.x.is_sign_positive());
        assert!(bb.0.y.is_infinite() && bb.0.y.is_sign_negative());
        assert!(bb.1.y.is_infinite() && bb.1.y.is_sign_positive());
        assert!(bb.0.z.is_infinite() && bb.0.z.is_sign_negative());
        assert!(bb.1.z.is_infinite() && bb.1.z.is_sign_positive());

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 1., 0.).normalize()),
            2.,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        approx::assert_abs_diff_eq!(bb.0, nalgebra_glm::DVec3::new(-(2.0_f64).sqrt(), -(2.0_f64).sqrt(), -1.));
        approx::assert_abs_diff_eq!(bb.1, nalgebra_glm::DVec3::new((2.0_f64).sqrt(), (2.0_f64).sqrt(), 1.));

        let cyl = Cylinder::new(
            Ray::new(nalgebra_glm::zero(), nalgebra_glm::DVec3::new(1., 0., 1.).normalize()),
            2.,
            1.,
            CylinderType::ThroughHole
        );
        let bb = cyl.bounding_box();
        approx::assert_abs_diff_eq!(bb.0, nalgebra_glm::DVec3::new(-(2.0_f64).sqrt(), -1., -(2.0_f64).sqrt()));
        approx::assert_abs_diff_eq!(bb.1, nalgebra_glm::DVec3::new((2.0_f64).sqrt(), 1., (2.0_f64).sqrt()));
    }
}