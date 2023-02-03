use crate::{
    common::Ray,
    scene::{
        obj::{
            SELFINTERSECTION_TOLERANCE,
            SceneObject,
            SceneObjectIntersection
        },
        storage::SceneObjectStorage
    }
};

pub enum Axis {
    X,
    Y,
    Z
}

impl Axis {
    pub fn extract_value(&self, vector: &nalgebra_glm::DVec3) -> f64 {
        match self {
            Axis::X => vector.x,
            Axis::Y => vector.y,
            Axis::Z => vector.z,
        }
    }
}

#[derive(Clone)]
pub struct AxisAlignedBoundingBox {
    min: nalgebra_glm::DVec3,
    max: nalgebra_glm::DVec3
}

impl AxisAlignedBoundingBox {
    pub fn new(min: nalgebra_glm::DVec3, max: nalgebra_glm::DVec3) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn new_infinity_box() -> Self {
        Self {
            min: nalgebra_glm::DVec3::from_element(std::f64::NEG_INFINITY),
            max: nalgebra_glm::DVec3::from_element(std::f64::INFINITY),
        }
    }

    pub fn unbounded(&self) -> bool {
        [self.min, self.max].iter()
            .any(|v| v.x.is_infinite() || v.y.is_infinite() || v.z.is_infinite())
    }

    pub fn centroid(&self) -> nalgebra_glm::DVec3 {
        (self.min + self.max) / 2.
    }

    pub fn largest_dimension(&self) -> Axis {
        [
            ((self.max.x - self.min.x).abs(), Axis::X),
            ((self.max.y - self.min.y).abs(), Axis::Y),
            ((self.max.z - self.min.z).abs(), Axis::Z)
        ].into_iter()
            .max_by(|(a, _), (b, _)| a.total_cmp(b))
            .map(|(_, a)| a)
            .expect("Iterator should never be empty.")
    }

    pub fn intersect(&self, ray: &Ray, closest: f64) -> bool {
        let inverted = nalgebra_glm::DVec3::from_element(1.).component_div(ray.direction());
        let dir_min = nalgebra_glm::DVec3::new(
            if ray.direction().x < 0. { self.max.x } else { self.min.x },
            if ray.direction().y < 0. { self.max.y } else { self.min.y },
            if ray.direction().z < 0. { self.max.z } else { self.min.z },
        );
        let dir_max = nalgebra_glm::DVec3::new(
            if ray.direction().x < 0. { self.min.x } else { self.max.x },
            if ray.direction().y < 0. { self.min.y } else { self.max.y },
            if ray.direction().z < 0. { self.min.z } else { self.max.z },
        );
        let tmin = (dir_min - ray.origin()).component_mul(&inverted);
        let tmax = (dir_max - ray.origin()).component_mul(&inverted);
        
        Some((tmin, tmax)).and_then(
            |(min, max)| {
                if min.x > max.y || min.y > max.x {
                    None
                } else {
                    Some((
                        if min.y > min.x { min.yz() } else { min.xz() },
                        if max.y < max.x { max.yz() } else { max.xz() }
                    ))
                }
            }
        ).and_then(
            |(min, max)| {
                if min.x > max.y || min.y > max.x {
                    None
                } else {
                    Some((
                        if min.y > min.x { min.y } else { min.x },
                        if max.y < max.x { max.y } else { max.x }
                    ))
                }
            }
        ).and_then(
            |(min, max)| {
                if min < closest && max > SELFINTERSECTION_TOLERANCE {
                    Some((min, max))
                } else {
                    None
                }
            }
        ).is_some()
    }

    /// Creates a new bounding box enclosing both
    pub fn enclose(&self, other: &Self) -> Self {
        Self {
            min: nalgebra_glm::min2(&self.min, &other.min),
            max: nalgebra_glm::max2(&self.max, &other.max),
        }
    }

    /// Creates a new bounding box enclosing box and point
    pub fn enclose_point(&self, point: &nalgebra_glm::DVec3) -> Self {
        Self {
            min: nalgebra_glm::min2(&self.min, point),
            max: nalgebra_glm::max2(&self.max, point),
        }
    }
}

impl From<(nalgebra_glm::DVec3, nalgebra_glm::DVec3)> for AxisAlignedBoundingBox {
    fn from(value: (nalgebra_glm::DVec3, nalgebra_glm::DVec3)) -> Self {
        let (min, max) = value;
        Self {
            min,
            max
        }
    }
}

pub enum BoundingVolumeHierarchyNode {
    Leaf { aabb: AxisAlignedBoundingBox, object_cout: usize, first_index: usize },
    Branch {
        aabb: AxisAlignedBoundingBox,
        axis: Axis,
        left: Box<Option<BoundingVolumeHierarchyNode>>,
        right: Box<Option<BoundingVolumeHierarchyNode>>,
    }
}

impl BoundingVolumeHierarchyNode {
    pub fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        match self {
            BoundingVolumeHierarchyNode::Leaf { aabb, object_cout: _, first_index: _ } => aabb,
            BoundingVolumeHierarchyNode::Branch { aabb, axis: _, left: _, right: _ } => aabb
        }
    }

    pub fn find_intersection<'a>(&'a self, ray: &Ray, objects: &'a [SceneObject], closest_int: f64) -> Option<SceneObjectIntersection> {
        if self.bounding_box().intersect(ray, closest_int) {
            match self {
                BoundingVolumeHierarchyNode::Leaf { aabb: _, object_cout, first_index } => {
                    let slc = &objects[*first_index..(first_index + object_cout)];
                    slc.iter()
                        .filter_map(|obj| obj.intersect(ray).map(|int| (obj, int.0, int.1)))
                        .filter(|(_, _, t)| t < &closest_int)
                        .filter(|(_, _, t)| t >= &SELFINTERSECTION_TOLERANCE)
                        .min_by(|(_, _, rt), (_, _, lt)| rt.total_cmp(lt))
                        .map(|(obj, n, t)| SceneObjectIntersection::<'a>::new(obj, n, t))
                },
                BoundingVolumeHierarchyNode::Branch {
                    aabb: _,
                    axis: _,
                    left,
                    right
                } => {
                    let left_int = left.as_ref().as_ref().and_then(|l| l.find_intersection(ray, objects, closest_int));
                    let t = match &left_int {
                        Some(a) => a.ray_length(),
                        None => closest_int
                    };
                    let right_int = right.as_ref().as_ref().and_then(|l| l.find_intersection(ray, objects, t));
                    if right_int.is_some() {
                        right_int
                    } else {
                        left_int
                    }
                }
            }
        } else {
            None
        }
    }
}

pub struct BoundingVolumeHierarchy {
    unbounded: Vec<SceneObject>,
    bounded: Vec<SceneObject>,
    tree: Option<BoundingVolumeHierarchyNode>,
    needs_rebuild: bool
}

impl BoundingVolumeHierarchy {
    pub fn new() -> Self {
        Self {
            unbounded: vec![],
            bounded: vec![],
            tree: None,
            needs_rebuild: false
        }
    }

    fn split_bvh(objects: &mut [SceneObject], first: usize, max_leaf_size: usize) -> Option<BoundingVolumeHierarchyNode> {
        if objects.len() <= max_leaf_size {
            match objects {
                [a] => Some(BoundingVolumeHierarchyNode::Leaf {
                    aabb: a.bounding_box().into(),
                    object_cout: 1,
                    first_index: first
                }),
                [a, b @ ..] => {
                    let aabb: AxisAlignedBoundingBox = b.iter().fold(
                        a.bounding_box().into(),
                        |state, next| {
                            state.enclose(&next.bounding_box().into())
                        }
                    );
                    Some(BoundingVolumeHierarchyNode::Leaf {
                        aabb,
                        object_cout: b.len() + 1,
                        first_index: first
                    })
                },
                [] => None
            }
        } else {
            let centroid_aabb = objects.iter()
                .fold(
                    AxisAlignedBoundingBox::from(objects[0].bounding_box()),
                    |state, next| state.enclose(&next.bounding_box().into())
                );
            let largest_axis = centroid_aabb.largest_dimension();
            objects.sort_by(
                |a, b| {
                    largest_axis.extract_value(&AxisAlignedBoundingBox::from(a.bounding_box()).centroid())
                        .total_cmp(&largest_axis.extract_value(&AxisAlignedBoundingBox::from(b.bounding_box()).centroid()))
                }
            );
            let (halfa, halfb) = objects.split_at_mut(objects.len() / 2);
            let left = Self::split_bvh(halfa, first, max_leaf_size);
            let right = Self::split_bvh(halfb, first + halfa.len(), max_leaf_size);
            let aabb = match (&left, &right) {
                (Some(a), Some(b)) => a.bounding_box().enclose(b.bounding_box()),
                (Some(a), None) => a.bounding_box().clone(),
                (None, Some(b)) => b.bounding_box().clone(),
                _ => panic!("At least one side should be some.")
            };
            Some(BoundingVolumeHierarchyNode::Branch {
                aabb,
                axis: largest_axis,
                left: Box::new(left),
                right: Box::new(right)
            })
        }
    }

    pub fn rebuild(&mut self, max_leaf_size: usize) {
        self.tree = Self::split_bvh(&mut self.bounded, 0, max_leaf_size);
        self.needs_rebuild = false;
    }
}

impl SceneObjectStorage for BoundingVolumeHierarchy {
    fn find_intersection(&self, ray: &Ray) -> Option<SceneObjectIntersection> {
        if self.needs_rebuild {
            None
        } else {
            let unb_int = self.unbounded.find_intersection(ray);
            let t = match &unb_int {
                Some(int) => int.ray_length(),
                None => std::f64::INFINITY
            };
            let b_int = self.tree.as_ref().and_then(
                |root| {
                    root.find_intersection(ray, &self.bounded, t)
                }
            );
            if b_int.is_some() {
                b_int
            } else {
                unb_int
            }
        }
    }

    fn insert_object(&mut self, obj: SceneObject) {
        let bb: AxisAlignedBoundingBox = obj.bounding_box().into();
        if bb.unbounded() {
            self.unbounded.push(obj);
        } else {
            self.bounded.push(obj);
            self.needs_rebuild = true;
        }
    }

    fn rebuild(&mut self) {
        self.rebuild(1)
    }
}

impl Default for BoundingVolumeHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<SceneObject>> for BoundingVolumeHierarchy {
    fn from(value: Vec<SceneObject>) -> Self {
        let mut bvh = Self::new();
        value.into_iter().for_each(|obj| bvh.insert_object(obj));
        bvh
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn enclose_test() {
        let a = AxisAlignedBoundingBox::new(
            nalgebra_glm::DVec3::new(5., 5., 5.),
            nalgebra_glm::DVec3::new(10., 15., 20.)
        );
        let b = AxisAlignedBoundingBox::new(
            nalgebra_glm::DVec3::new(0., 0., 0.),
            nalgebra_glm::DVec3::new(15., 10., 15.)
        );
        let enc = a.enclose(&b);
        let expect = AxisAlignedBoundingBox::new(
            nalgebra_glm::DVec3::new(0., 0., 0.),
            nalgebra_glm::DVec3::new(15., 15., 20.)
        );
        assert_eq!(enc.min, expect.min);
        assert_eq!(enc.max, expect.max);
    }

    #[test]
    fn enclose_point_test() {
        let a = AxisAlignedBoundingBox::new(
            nalgebra_glm::DVec3::new(5., 5., 5.),
            nalgebra_glm::DVec3::new(10., 15., 20.)
        );
        let b = nalgebra_glm::DVec3::new(0., 0., 0.);
        let enc = a.enclose_point(&b);
        let expect = AxisAlignedBoundingBox::new(
            nalgebra_glm::DVec3::new(0., 0., 0.),
            nalgebra_glm::DVec3::new(10., 15., 20.)
        );
        assert_eq!(enc.min, expect.min);
        assert_eq!(enc.max, expect.max);

        let b = nalgebra_glm::DVec3::new(20., 0., 0.);
        let enc = a.enclose_point(&b);
        let expect = AxisAlignedBoundingBox::new(
            nalgebra_glm::DVec3::new(5., 0., 0.),
            nalgebra_glm::DVec3::new(20., 15., 20.)
        );
        assert_eq!(enc.min, expect.min);
        assert_eq!(enc.max, expect.max);

        let b = nalgebra_glm::DVec3::new(20., 20., 20.);
        let enc = a.enclose_point(&b);
        let expect = AxisAlignedBoundingBox::new(
            nalgebra_glm::DVec3::new(5., 5., 5.),
            nalgebra_glm::DVec3::new(20., 20., 20.)
        );
        assert_eq!(enc.min, expect.min);
        assert_eq!(enc.max, expect.max);
    }
}