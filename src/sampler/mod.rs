mod halton_sampler;
pub use halton_sampler::HaltonSampler;

mod random_sampler;
pub use random_sampler::RandomSampler;

/// Generaters new directions to sample
pub trait Sampler: std::marker::Sync {
    fn hemisphere(&self) -> glm::DVec3;
}