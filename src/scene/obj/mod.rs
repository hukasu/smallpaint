use crate::common::Ray;

mod plane;
pub use plane::Plane;

mod sphere;
pub use sphere::Sphere;

pub const SELFINTERSECTION_TOLERANCE: f64 = 1e-6;

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