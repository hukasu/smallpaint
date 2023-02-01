use crate::{
    scene::{
        Scene,
        obj::{SceneObject, SceneObjectMaterial, CylinderType}, sample::SampleScene
    },
    common::Ray
};

pub struct RingCaustics;

impl SampleScene for RingCaustics {
    fn build_sample_scene() -> Scene {
        const BASE_EMISSION: f64 = 0.;
        const LIGHT_EMISSION: f64 = 5_000.;

        let mut rscene: Scene = Scene::new_with_vec_storage();

        // Room walls
        rscene.insert_object(
            SceneObject::new_plane(
                nalgebra_glm::DVec3::new(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(-3., 0., 0.),
                nalgebra_glm::DVec3::new(1., 0., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                nalgebra_glm::DVec3::new(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(2.5, 0., 0.),
                nalgebra_glm::DVec3::new(-1., 0., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                nalgebra_glm::DVec3::new(10., 2., 2.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(0., -2.75, 0.),
                nalgebra_glm::DVec3::new(0., 1., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                nalgebra_glm::DVec3::new(2., 10., 2.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(0., 2.75, 0.),
                nalgebra_glm::DVec3::new(0., -1., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                nalgebra_glm::DVec3::new(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(0., 0., -5.5),
                nalgebra_glm::DVec3::new(0., 0., 1.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                nalgebra_glm::DVec3::new(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(0., 0., 0.5),
                nalgebra_glm::DVec3::new(0., 0., -1.)
            )
        );
        // Light
        rscene.insert_object(
            SceneObject::new_sphere(
                nalgebra_glm::DVec3::new(0., 0., 0.),
                LIGHT_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(1.5, 0., -4.5),
                0.5
            )
        );
        // Ring
        rscene.insert_object(
            SceneObject::new_cylinder(
                nalgebra_glm::DVec3::new(4., 4., 4.),
                BASE_EMISSION,
                SceneObjectMaterial::Specular,
                Ray::new(nalgebra_glm::DVec3::new(-1., 0., -5.5), nalgebra_glm::DVec3::new(0., 0., 1.)),
                0.5,
                1.0,
                CylinderType::ThroughHole
            ).expect("Following constraints")
        );

        rscene
    }
}