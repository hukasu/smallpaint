use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, Plane}
};

use super::SELFINTERSECTION_TOLERANCE;

pub enum CylinderType {
    ThroughHole,
    SingleCap,
    DoubleCap
}

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

    fn surface_intersection(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        let rdcd = glm::dot(*ray.direction(), *self.axis.direction());
        if glm::is_approx_eq(&rdcd, &0.) {
            // Skew lines
            let n = glm::cross(*self.axis.direction(), *ray.direction());
            let n2 = glm::cross(*ray.direction(), n);
            let n1 = glm::cross(*self.axis.direction(), n);
            // Point on the axis of the cylinder closest to ray
            let cp = *self.axis.origin() + *self.axis.direction() * (glm::dot(*ray.origin() - *self.axis.origin(), n2)/(glm::dot(*self.axis.direction(), n2)));
            if glm::distance(cp, *self.axis.origin()) < self.height / 2. {
                // Point on the ray closest to axis of the cylinder
                let rp = *ray.origin() + *ray.direction() * (glm::dot(*self.axis.origin() - *ray.origin(), n1)/(glm::dot(*ray.direction(), n1)));
                let dist = glm::distance(cp, rp);
                if dist < self.radius {
                    let displ = (self.radius.powi(2) - dist.powi(2)).sqrt();
                    [rp - ((*ray.direction() * displ)), rp + ((*ray.direction() * displ))].into_iter()
                        .filter_map(
                            |hp| {
                                let t = glm::distance(hp, *ray.origin());
                                if t > SELFINTERSECTION_TOLERANCE {
                                    Some((glm::normalize(hp - cp), t))
                                } else {
                                    None
                                }
                            }
                        )
                        .min_by(|(_, a), (_, b)| a.total_cmp(b))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            let roro = glm::dot(*ray.origin(), *ray.origin());
            let rord = glm::dot(*ray.origin(), *ray.direction());
            let roco = glm::dot(*ray.origin(), *self.axis.origin());
            let rocd = glm::dot(*ray.origin(), *self.axis.direction());
            let rdco = glm::dot(*ray.direction(), *self.axis.origin());
            let coco = glm::dot(*self.axis.origin(), *self.axis.origin());
            let cocd = glm::dot(*self.axis.origin(), *self.axis.direction());

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
                                    let axis_point = *self.axis.origin() + *self.axis.direction() * m;
                                    let hit_point = *ray.origin() + *ray.direction() * t;
                                    Some((glm::normalize(hit_point - axis_point), t))
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
    }

    fn cap_intersection(&self, ray: &Ray, top_cap: bool) -> Option<(glm::DVec3, f64)> {
        let p = if top_cap {
            Plane::new(*self.axis.origin() + (*self.axis.direction() * (self.height / 2.)), *self.axis.direction())
        } else {
            Plane::new(*self.axis.origin() - (*self.axis.direction() * (self.height / 2.)), *self.axis.direction() * -1.)
        };
        match (&self.ctype, top_cap) {
            (CylinderType::DoubleCap, _) | (CylinderType::SingleCap, false) => {
                p.intersect(ray)
                    .filter(
                        |int| {
                            let hp = *ray.origin() + *ray.direction() * int.1;
                            if glm::distance(*p.point(), hp) < self.radius {
                                true
                            } else {
                                false
                            }
                        }
                    )
            },
            _ => None
        }
    }
}

impl SceneObjectGeometry for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        let tests = [
            self.surface_intersection(ray),
            self.cap_intersection(ray, true),
            self.cap_intersection(ray, false)
        ];
        tests.into_iter()
            .filter_map(|t| t)
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
    }

    fn bounding_box(&self) -> (glm::DVec3, glm::DVec3) {
        todo!()
    }
}