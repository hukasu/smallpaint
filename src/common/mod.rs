mod ray;
pub use ray::Ray;

pub struct RandomGen;

impl RandomGen {
    pub fn rand() -> f64 {
        2. * rand::random::<f64>() - 1.
    }

    pub fn rand2() -> f64 {
        rand::random::<f64>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rand_within_range() {
        for _ in 0..100_000_000 {
            let r = RandomGen::rand();
            assert!(r >= -1. && r <= 1., "rand() = {}", r);
        }
    }

    #[test]
    fn rand2_within_range() {
        for _ in 0..100_000_000 {
            let r = RandomGen::rand2();
            assert!(r >= 0. && r <= 1., "rand2() = {}", r);
        }
    }
}