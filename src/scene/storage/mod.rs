mod bvh;
pub use bvh::{AxisAlignedBoundingBox, Axis, BoundingVolumeHierarchy};

use crate::{scene::obj::{SceneObject, SceneObjectIntersection, SELFINTERSECTION_TOLERANCE}, common::Ray};
pub trait SceneObjectStorage: std::marker::Sync {
    fn find_intersection(&self, ray: &Ray) -> Option<SceneObjectIntersection>;
    fn insert_object(&mut self, obj: SceneObject);
    fn rebuild(&mut self);
}

impl SceneObjectStorage for Vec<SceneObject> {
    fn find_intersection(&self, ray: &Ray) -> Option<SceneObjectIntersection> {
        self.iter()
            .filter_map(|obj| obj.intersect(ray).map(|int| (obj, int.0, int.1)))
            .filter(|(_, _, d)| d >= &SELFINTERSECTION_TOLERANCE)
            .min_by(|(_, _, rd), (_, _, ld)| rd.total_cmp(ld))
            .map(|(obj, n, d)| SceneObjectIntersection::new(obj, n, d))
    }

    fn insert_object(&mut self, obj: SceneObject) {
        self.push(obj)
    }

    fn rebuild(&mut self) {}
}