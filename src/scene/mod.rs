use crate::common::Ray;

pub mod obj;

pub trait SceneObjectStorage: std::marker::Sync {
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
            objects: Box::new(Vec::new())
        }
    }

    pub fn find_intersection(&self, ray: &Ray) -> Option<obj::SceneObjectIntersection> {
        self.objects.find_intersection(ray)
    }

    pub fn insert_object(&mut self, object: obj::SceneObject) {
        self.objects.insert_object(object)
    }
}