mod three_spheres;
pub use three_spheres::ThreeSpheresSampleScene;

mod three_cylinders_with_lights;
pub use three_cylinders_with_lights::ThreeCylindersWithLightsSampleScene;

use crate::scene::Scene;

pub trait SampleScene {
    fn build_sample_scene() -> Scene;
}