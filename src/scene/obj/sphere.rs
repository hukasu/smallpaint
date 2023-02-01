use crate::{
    common::Ray,
    scene::obj::{SceneObjectGeometry, SELFINTERSECTION_TOLERANCE}
};

#[derive(Debug)]
pub struct Sphere {
    center: nalgebra_glm::DVec3,
    radius: f64
}

impl Sphere {
    pub fn new(
        center: nalgebra_glm::DVec3,
        radius: f64
    ) -> Self {
        Self {
            center,
            radius
        }
    }

    pub fn center(&self) -> &nalgebra_glm::DVec3 {
        &self.center
    } 

    pub fn radius(&self) -> f64 {
        self.radius
    } 
}

impl SceneObjectGeometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(nalgebra_glm::DVec3, f64)> {
        let ray_mns_center = *ray.origin() - self.center;
        let b = (ray_mns_center * 2.).dot(ray.direction());
        let c = ray_mns_center.dot(&ray_mns_center) - self.radius.powf(2.);
        let disc = b.powf(2.) - 4. * c;
        if disc >= 0. {
            let disc = disc.sqrt();
            let sol1 = -b + disc;
            let sol2 = -b - disc;
            if sol2 > SELFINTERSECTION_TOLERANCE {
                let t = sol2 / 2.;
                let intersect = *ray.origin() + *ray.direction() * t;
                Some(((intersect - self.center).normalize(), t))
            }
            else if sol1 > SELFINTERSECTION_TOLERANCE {
                let t = sol1 / 2.;
                let intersect = *ray.origin() + *ray.direction() * t;
                Some(((intersect - self.center).normalize(), t))
            }
            else { None }
        } else { None }
    }

    fn bounding_box(&self) -> (nalgebra_glm::DVec3, nalgebra_glm::DVec3) {
        (
            nalgebra_glm::DVec3::new(
                self.center.x - self.radius,
                self.center.y - self.radius,
                self.center.z - self.radius,
            ),
            nalgebra_glm::DVec3::new(
                self.center.x + self.radius,
                self.center.y + self.radius,
                self.center.z + self.radius,
            ),
        )
    }
}