use crate::scene::{Scene, obj::{SceneObject, SceneObjectMaterial}, sample::SampleScene};

pub struct ThreeSpheresSampleScene;

impl SampleScene for ThreeSpheresSampleScene {
    fn build_sample_scene() -> Scene {
        const BASE_EMISSION: f64 = 0.;
        const LIGHT_EMISSION: f64 = 5_000.;

        let mut rscene: Scene = Scene::new_with_vec_storage();

        rscene.insert_object(
            SceneObject::new_sphere(
                glm::dvec3(4., 8., 4.),
                BASE_EMISSION,
                SceneObjectMaterial::Specular,
                glm::dvec3(1.45, -0.75, -4.4),
                1.05
            )
        );
        rscene.insert_object(
            SceneObject::new_sphere(
                glm::dvec3(10., 10., 1.),
                BASE_EMISSION,
                SceneObjectMaterial::Refractive,
                glm::dvec3(2.05, 2.0, -3.7),
                0.5
            )
        );
        rscene.insert_object(
            SceneObject::new_sphere(
                glm::dvec3(4., 4., 12.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(1.95, -1.75, -3.1),
                0.6
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(1., 0., 0.),
                3.0
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(-1., 0., 0.),
                2.5
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(10., 2., 2.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., 1., 0.),
                2.75
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(2., 10., 2.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., -1., 0.),
                2.75
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., 0., 1.),
                5.5
            )
        );
        rscene.insert_object(
            SceneObject::new_plane(
                glm::dvec3(6., 6., 6.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(0., 0., -1.),
                0.5
            )
        );
        rscene.insert_object(
            SceneObject::new_sphere(
                glm::dvec3(0., 0., 0.),
                LIGHT_EMISSION,
                SceneObjectMaterial::Diffuse,
                glm::dvec3(-1.9, 0., -3.),
                0.5
            )
        );

        rscene
    }
}