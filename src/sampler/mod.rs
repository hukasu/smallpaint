mod halton_sampler;
pub use halton_sampler::HaltonSampler;

/// Generaters new directions to sample
pub trait Sampler: std::marker::Sync {
    fn hemisphere(&self) -> glm::DVec3;
}