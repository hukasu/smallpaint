use crate::common::Ray;

pub mod obj;

use self::obj::SceneObject;

pub mod storage;
use storage::{SceneObjectStorage, BoundingVolumeHierarchy};

#[cfg(feature = "sample-scenes")]
pub mod sample;

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

    pub fn new_with_bounding_volume_hierarchy() -> Self {
        Self {
            objects: Box::<BoundingVolumeHierarchy>::default()
        }
    }

    pub fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection> {
        self.objects.find_intersection(ray)
    }

    pub fn insert_object(&mut self, object: obj::SceneObject) {
        self.objects.insert_object(object)
    }

    /// Rebuilds storage, some storage types require rebuilding after changes to its contents.
    /// 
    /// Rebuilds with default parameters, if you wish to rebuild the storage with custom parameters,
    /// construct and rebuild the storage before attaching it to the `Scene`.
    pub fn rebuild_storage(&mut self) {
        self.objects.rebuild()
    }
}