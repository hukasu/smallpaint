use crate::common::RandomGen;
use super::Sampler;
pub struct RandomSampler;

impl RandomSampler {
    pub fn new() -> Self {
        Self
    }

    fn next(&self) -> (f64, f64) {
        (RandomGen::rand2(), RandomGen::rand2())
    }
}

impl Sampler for RandomSampler {
    fn hemisphere(&self) -> glm::DVec3 {
        let (u1, u2) = self.next();
        let r = (1.0 - u1.powi(2)).sqrt();
        let phi = 2. * std::f64::consts::PI * u2;
        glm::dvec3(phi.cos() * r, phi.sin() * r, u1)
    }
}