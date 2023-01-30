use crate::common::Ray;

pub mod obj;
use obj::SELFINTERSECTION_TOLERANCE;

use self::obj::SceneObject;

#[cfg(feature = "sample-scenes")]
pub mod sample;

pub trait SceneObjectStorage: std::marker::Sync {
    fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection>;
    fn insert_object(&mut self, obj: obj::SceneObject);
}

impl SceneObjectStorage for Vec<obj::SceneObject> {
    fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection> {
        self.iter()
            .filter_map(|obj| obj.intersect(ray).map(|int| (obj, int.0, int.1)))
            .filter(|(_, _, d)| d >= &SELFINTERSECTION_TOLERANCE)
            .min_by(|(_, _, rd), (_, _, ld)| rd.total_cmp(ld))
            .map(|(obj, n, d)| obj::SceneObjectIntersection::new(obj, n, d))
    }

    fn insert_object(&mut self, obj: obj::SceneObject) {
        self.push(obj)
    }
}

pub struct Scene {
    objects: Box<dyn SceneObjectStorage>
}

impl Scene {
    pub fn new(objects: Box<dyn SceneObjectStorage>) -> Self {
        Self {
            objects
        }
    }

    pub fn new_with_vec_storage() -> Self {
        Self {
            objects: Box::<Vec<SceneObject>>::default()
        }
    }

    pub fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection> {
        self.objects.find_intersection(ray)
    }

    pub fn insert_object(&mut self, object: obj::SceneObject) {
        self.objects.insert_object(object)
    }
}