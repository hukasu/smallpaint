use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, SELFINTERSECTION_TOLERANCE}
};

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
}

impl SceneObjectGeometry for Sphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        let ray_mns_center = *ray.origin() - self.center;
        let b = glm::dot(ray_mns_center * glm::to_dvec3(2.), *ray.direction());
        let c = glm::dot(ray_mns_center, ray_mns_center) - (self.radius.powf(2.));
        let disc = b.powf(2.) - 4. * c;
        if disc >= 0. {
            let disc = disc.sqrt();
            let sol1 = -b + disc;
            let sol2 = -b - disc;
            if sol2 > SELFINTERSECTION_TOLERANCE { sol2 / 2. }
            else {
                if sol1 > SELFINTERSECTION_TOLERANCE { sol1 / 2. }
                else { 0. }
            }
        } else { 0. }
    }

    fn normal(&self, intersect: &glm::DVec3) -> glm::DVec3 {
        glm::normalize(*intersect - self.center)
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