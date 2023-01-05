use crate::common::Ray;

pub mod obj;

pub trait SceneObjectStorage: Default + std::marker::Sync {
    fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection>;
    fn insert_object(&mut self, obj: obj::SceneObject);
}

impl SceneObjectStorage for Vec<obj::SceneObject> {
    fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection> {
        self.iter()
            .filter_map(|obj| Some((obj, obj.intersect(ray))))
            .filter(|(_, d)| d >= &1e-6)
            .min_by(|(_, rd), (_, ld)| rd.total_cmp(ld))
            .map(|(obj, d)| obj::SceneObjectIntersection::new(obj, d))
    }

    fn insert_object(&mut self, obj: obj::SceneObject) {
        self.push(obj)
    }
}

pub struct Scene<T> where T: SceneObjectStorage {
    objects: T
}

impl<T> Scene<T> where T: SceneObjectStorage {
    pub fn new() -> Self {
        Self {
            objects: T::default()
        }
    }

    pub fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection> {
        self.objects.find_intersection(ray)
    }

    pub fn insert_object(&mut self, object: obj::SceneObject) {
        self.objects.insert_object(object)
    }
}