trait OrthonormalVectorExt<V>: glm::GenFloatVec<V>
where V: glm::BaseFloat {
    fn orthonormal(&self) -> (glm::Vector3<V>, glm::Vector3<V>);
}

impl<V> OrthonormalVectorExt<V> for glm::Vector3<V>
where V: glm::BaseFloat + glm::GenFloat<V> {
    fn orthonormal(&self) -> (glm::Vector3<V>, glm::Vector3<V>) {
        let v = glm::normalize(*self);
        let va = if glm::is_approx_eq(&v, &glm::Vector3::new(V::one(), V::zero(), V::zero())) {
            glm::cross(v, glm::Vector3::new(V::zero(), V::one(), V::zero()))
        } else {
            glm::cross(v, glm::Vector3::new(V::one(), V::zero(), V::zero()))
        };
        let vb = glm::cross(v, va);
        (va, vb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glm::is_approx_eq;

    fn orthonormal_unit_vector_common_test(v: &glm::DVec3) {
        let (v2, v3) = v.orthonormal();
        glm::assert_approx_eq!(glm::dot(*v, v2), 0.);
        glm::assert_approx_eq!(glm::dot(*v, v3), 0.);
        glm::assert_approx_eq!(glm::dot(v2, v3), 0.);
    }

    #[test]
    fn orthonormal_unit_vector_test() {
        orthonormal_unit_vector_common_test(&glm::dvec3(1.0, 0., 0.));
        orthonormal_unit_vector_common_test(&glm::dvec3(0.0, 1.0, 0.0));
        orthonormal_unit_vector_common_test(&glm::dvec3(0., 0., 1.));
    }

    #[test]
    fn orthonormal_example_test() {
        let v1 = glm::dvec3(1. / (2.0_f64).sqrt(), 0., -1. / (2.0_f64).sqrt());
        let v2 = glm::dvec3(0.5, (2.0_f64).sqrt() / 2., 0.5);
        let v3 = glm::dvec3(0.5, -(2.0_f64).sqrt() / 2., 0.5);
        glm::assert_approx_eq!(glm::dot(v1, v2), 0.);
        glm::assert_approx_eq!(glm::dot(v1, v3), 0.);
        glm::assert_approx_eq!(glm::dot(v2, v3), 0.);
    }
}