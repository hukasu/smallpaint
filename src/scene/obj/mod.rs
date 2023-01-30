use crate::common::Ray;

mod plane;
pub use plane::Plane;

mod sphere;
pub use sphere::Sphere;

mod cylinder;
pub use cylinder::{Cylinder, CylinderType};

mod lens;
pub use lens::Lens;

pub const SELFINTERSECTION_TOLERANCE: f64 = 1e-6;

#[derive(Debug)]
pub enum SceneObjectError {
    RefractiveCylinderConstraintError,
    LensFacesTooShortError,
    LensTooThinError,
    LensConcaveFaceTooDeepError
}

impl std::fmt::Display for SceneObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self {
            SceneObjectError::RefractiveCylinderConstraintError => String::from("A refractive cylinder must be Double Capped."),
            SceneObjectError::LensFacesTooShortError => String::from("One of the faces of the lens has an absolute radius smaller than the the radius of the lens."),
            SceneObjectError::LensTooThinError => String::from("The lens is too thin."),
            SceneObjectError::LensConcaveFaceTooDeepError => String::from("A concave face is too deep. The concave face can't have a depth too close to half of the thickness.")
        };
        writeln!(f, "{m}")
    }
}

impl std::error::Error for SceneObjectError {}

pub struct SceneObjectIntersection<'a> {
    object: &'a SceneObject,
    normal: glm::DVec3,
    ray_length: f64
}

impl<'a> SceneObjectIntersection<'a> {
    pub fn new(
        object: &'a SceneObject,
        normal: glm::DVec3,
        ray_length: f64
    ) -> Self {
        Self {
            object,
            normal,
            ray_length
        }
    }

    pub fn object(&self) -> &SceneObject {
        self.object
    }

    pub fn normal(&self) -> glm::DVec3 {
        self.normal
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
        point: glm::DVec3,
        normal: glm::DVec3,
    ) -> Self {
        Self {
            color,
            emission,
            material,
            geometry: Box::new(
                Plane::new(
                    point,
                    normal
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

    pub fn new_cylinder(
        color: glm::DVec3,
        emission: f64,
        material: SceneObjectMaterial,
        axis: Ray,
        height: f64,
        radius: f64,
        ctype: CylinderType
    ) -> Result<Self, SceneObjectError> {
        if matches!(material, SceneObjectMaterial::Refractive) && !matches!(ctype, CylinderType::DoubleCap) {
            Err(SceneObjectError::RefractiveCylinderConstraintError)
        } else {
            Ok(     
                Self {
                    color,
                    emission,
                    material,
                    geometry: Box::new(
                        Cylinder::new(
                            axis,
                            height,
                            radius,
                            ctype
                        )
                    )
                }
            )
        }
    }

    pub fn new_lens(
        color: glm::DVec3,
        emission: f64,
        material: SceneObjectMaterial,
        axis: Ray,
        thickness: f64,
        radius: f64,
        front_radius: f64,
        back_radius: f64
    ) -> Result<Self, SceneObjectError> {
        let lens = Lens::new(
            axis,
            thickness,
            radius,
            front_radius,
            back_radius
        )?;
        Ok(
            Self {
                color,
                emission,
                material,
                geometry: Box::new(lens)
            }
        )
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

    pub fn intersect(&self, ray: &Ray) -> Option<(glm::DVec3, f64)> {
        self.geometry.intersect(ray)
    }

    pub fn bounding_box(&self) -> (glm::DVec3, glm::DVec3) {
        self.geometry.bounding_box()
    }
}

pub trait SceneObjectGeometry: std::marker::Sync {
    fn intersect(&self, ray: &Ray) -> Option<(glm::DVec3, f64)>;
    fn bounding_box(&self) -> (glm::DVec3, glm::DVec3);
}