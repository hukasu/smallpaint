use crate::common::Ray;

pub struct SceneObjectIntersection<'a> {
    object: &'a SceneObject,
    ray_length: f64
}

impl<'a> SceneObjectIntersection<'a> {
    pub fn new(
        object: &'a SceneObject,
        ray_length: f64
    ) -> Self {
        Self {
            object,
            ray_length
        }
    }

    pub fn object(&self) -> &SceneObject {
        &self.object
    }

    pub fn ray_length(&self) -> f64 {
        self.ray_length
    }
}

#[derive(Clone, Copy)]
pub enum SceneObjectMaterial {
    Diffuse,
    Specular,
    Refractive
} 

pub struct SceneObject {
    color: glm::DVec3,
    emission: f64,
    material: SceneObjectMaterial,
    geometry: Box<dyn SceneObjectGeometry>
}

impl SceneObject {
    pub fn new(
        color: glm::DVec3,
        emission: f64,
        material: SceneObjectMaterial,
        geometry: Box<dyn SceneObjectGeometry>
    ) -> Self {
        Self {
            color,
            emission,
            material,
            geometry
        }
    }

    pub fn new_plane(
        color: glm::DVec3,
        emission: f64,
        material: SceneObjectMaterial,
        normal: glm::DVec3,
        displacement: f64
    ) -> Self {
        Self {
            color,
            emission,
            material,
            geometry: Box::new(
                Plane::new(
                    normal,
                    displacement
                )
            )
        }
    }

    pub fn new_sphere(
        color: glm::DVec3,
        emission: f64,
        material: SceneObjectMaterial,
        center: glm::DVec3,
        radius: f64
    ) -> Self {
        Self {
            color,
            emission,
            material,
            geometry: Box::new(
                Sphere::new(
                    center,
                    radius
                )
            )
        }
    }

    pub fn color(&self) -> &glm::DVec3 {
        &self.color
    }

    pub fn emission(&self) -> f64 {
        self.emission
    }

    pub fn material(&self) -> SceneObjectMaterial {
        self.material
    }

    pub fn intersect(&self, ray: &Ray) -> f64 {
        self.geometry.intersect(ray)
    }

    pub fn normal(&self, intersect: &glm::DVec3) -> glm::DVec3 {
        self.geometry.normal(intersect)
    }

    pub fn bounding_box(&self) -> (glm::DVec3, glm::DVec3) {
        self.geometry.bounding_box()
    }
}

pub trait SceneObjectGeometry: std::marker::Sync {
    fn intersect(&self, ray: &Ray) -> f64;
    fn normal(&self, intersect: &glm::DVec3) -> glm::DVec3;
    fn bounding_box(&self) -> (glm::DVec3, glm::DVec3); // TODO
}

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
            if sol2 > 1e-6 { sol2 / 2. }
            else {
                if sol1 > 1e-6 { sol1 / 2. }
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