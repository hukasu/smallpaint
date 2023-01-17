mod three_spheres;
pub use three_spheres::ThreeSpheresSampleScene;

use crate::scene::Scene;

pub trait SampleScene {
    fn build_sample_scene() -> Scene;
}