use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, SELFINTERSECTION_TOLERANCE}
};

#[derive(Debug)]
pub struct Sphere {
    center: glm::DVec3,
    radius: f64
}

impl Sphere {
    pub fn new(
        center: glm::DVec3,
        radius: f64
    ) -> Self {
        Self {
            center,
            radius
        }
    }

    pub fn center(&self) -> glm::DVec3 {
        self.center
    } 

    pub fn radius(&self) -> f64 {
        self.radius
    } 
}

impl SceneObjectGeometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        let ray_mns_center = *ray.origin() - self.center;
        let b = glm::dot(ray_mns_center * glm::to_dvec3(2.), *ray.direction());
        let c = glm::dot(ray_mns_center, ray_mns_center) - (self.radius.powf(2.));
        let disc = b.powf(2.) - 4. * c;
        if disc >= 0. {
            let disc = disc.sqrt();
            let sol1 = -b + disc;
            let sol2 = -b - disc;
            if sol2 > SELFINTERSECTION_TOLERANCE {
                let t = sol2 / 2.;
                let intersect = *ray.origin() + *ray.direction() * t;
                Some((glm::normalize(intersect - self.center), t))
            }
            else if sol1 > SELFINTERSECTION_TOLERANCE {
                let t = sol1 / 2.;
                let intersect = *ray.origin() + *ray.direction() * t;
                Some((glm::normalize(intersect - self.center), t))
            }
            else { None }
        } else { None }
    }

    fn bounding_box(&self) -> (glm::DVec3, glm::DVec3) {
        (
            glm::dvec3(
                self.center.x - self.radius,
                self.center.y - self.radius,
                self.center.z - self.radius,
            ),
            glm::dvec3(
                self.center.x + self.radius,
                self.center.y + self.radius,
                self.center.z + self.radius,
            ),
        )
    }
}