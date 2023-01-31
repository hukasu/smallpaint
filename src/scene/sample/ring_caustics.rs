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
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(-3., 0., 0.),
                glm::dvec3(1., 0., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(2.5, 0., 0.),
                glm::dvec3(-1., 0., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(10., 2., 2.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., -2.75, 0.),
                glm::dvec3(0., 1., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(2., 10., 2.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., 2.75, 0.),
                glm::dvec3(0., -1., 0.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., 0., -5.5),
                glm::dvec3(0., 0., 1.)
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., 0., 0.5),
                glm::dvec3(0., 0., -1.)
            )
        );
        // Light
        rscene.insert_object(
            SceneObject::new_sphere(
                glm::dvec3(0., 0., 0.),
                LIGHT_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(1.5, 0., -4.5),
                0.5
            )
        );
        // Ring
        rscene.insert_object(
            SceneObject::new_cylinder(
                glm::dvec3(4., 4., 4.),
                BASE_EMISSION,
                SceneObjectMaterial::Specular,
                Ray::new(glm::dvec3(-1., 0., -5.5), glm::dvec3(0., 0., 1.)),
                0.5,
                1.0,
                CylinderType::ThroughHole
            ).expect("Following constraints")
        );

        rscene
    }
}