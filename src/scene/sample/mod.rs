mod three_spheres;
pub use three_spheres::ThreeSpheresSampleScene;

mod three_cylinders_with_lights;
pub use three_cylinders_with_lights::ThreeCylindersWithLightsSampleScene;

mod lenses_and_bars;
pub use lenses_and_bars::LensesAndBars;

use crate::scene::Scene;

pub trait SampleScene {
    fn build_sample_scene() -> Scene;
}