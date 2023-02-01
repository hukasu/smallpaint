pub trait OrthonormalVectorExt<V>
where V: nalgebra_glm::RealNumber {
    fn orthonormal(&self) -> (nalgebra::Matrix3x1<V>, nalgebra::Matrix3x1<V>);
}

impl<V> OrthonormalVectorExt<V> for nalgebra::Matrix3x1<V>
where V: nalgebra_glm::RealNumber {
    fn orthonormal(&self) -> (nalgebra::Matrix3x1<V>, nalgebra::Matrix3x1<V>) {
        let v = self.normalize();
        let dot = v.dot(&nalgebra::Matrix3x1::new(V::one(), V::zero(), V::zero()));
        let va = if approx::abs_diff_eq!(dot.abs(), V::one()) {
            v.cross(&nalgebra::Matrix3x1::new(V::zero(), V::one(), V::zero()))
        } else {
            v.cross(&nalgebra::Matrix3x1::new(V::one(), V::zero(), V::zero()))
        };
        let vb = v.cross(&va);
        (va, vb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn orthonormal_unit_vector_common_test(v1: &nalgebra_glm::DVec3) {
        let (v2, v3) = v1.orthonormal();
        approx::assert_abs_diff_eq!(v1.dot(&v2), 0.);
        approx::assert_abs_diff_eq!(v1.dot(&v3), 0.);
        approx::assert_abs_diff_eq!(v2.dot(&v3), 0.);
    }

    #[test]
    fn orthonormal_unit_vector_test() {
        orthonormal_unit_vector_common_test(&nalgebra_glm::DVec3::new(1., 0., 0.));
        orthonormal_unit_vector_common_test(&nalgebra_glm::DVec3::new(0., 1., 0.));
        orthonormal_unit_vector_common_test(&nalgebra_glm::DVec3::new(0., 0., 1.));
        orthonormal_unit_vector_common_test(&nalgebra_glm::DVec3::new(-1., 0., 0.));
        orthonormal_unit_vector_common_test(&nalgebra_glm::DVec3::new(0., -1., 0.));
        orthonormal_unit_vector_common_test(&nalgebra_glm::DVec3::new(0., 0., -1.));
    }

    #[test]
    fn orthonormal_example_test() {
        let v1 = nalgebra_glm::DVec3::new(1. / (2.0_f64).sqrt(), 0., -1. / (2.0_f64).sqrt());
        let v2 = nalgebra_glm::DVec3::new(0.5, (2.0_f64).sqrt() / 2., 0.5);
        let v3 = nalgebra_glm::DVec3::new(0.5, -(2.0_f64).sqrt() / 2., 0.5);
        approx::assert_abs_diff_eq!(v1.dot(&v2), 0.);
        approx::assert_abs_diff_eq!(v1.dot(&v3), 0.);
        approx::assert_abs_diff_eq!(v2.dot(&v3), 0.);
    }
}