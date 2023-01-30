use crate::{
    scene::{
        Scene,
        obj::{SceneObject, SceneObjectMaterial, CylinderType}, sample::SampleScene
    },
    common::Ray
};

pub struct LensesAndBars;

impl SampleScene for LensesAndBars {
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
                glm::dvec3(-1.9, 0., -3.),
                0.5
            )
        );
        // Lenses
        rscene.insert_object(
            SceneObject::new_lens(
                glm::dvec3(0., 0., 0.),
                BASE_EMISSION,
                SceneObjectMaterial::Refractive,
                Ray::new(
                    glm::dvec3(0., 0., -3.),
                    glm::dvec3(0., 0., -1.)),
                1. / 16.,
                0.5,
                1.,
                1.
            ).unwrap()
        );
        rscene.insert_object(
            SceneObject::new_lens(
                glm::dvec3(0., 0., 0.),
                BASE_EMISSION,
                SceneObjectMaterial::Refractive,
                Ray::new(
                    glm::dvec3(0., -1., -3.),
                    glm::dvec3(0., 0., -1.)),
                1. / 16.,
                0.5,
                1.,
                -5.
            ).unwrap()
        );
        rscene.insert_object(
            SceneObject::new_lens(
                glm::dvec3(0., 0., 0.),
                BASE_EMISSION,
                SceneObjectMaterial::Refractive,
                Ray::new(
                    glm::dvec3(0., 1., -3.),
                    glm::dvec3(0., 0., -1.)),
                1. / 16.,
                0.5,
                -5.,
                -5.
            ).unwrap()
        );
        // Bars
        let mut colors = [
            glm::dvec3(4., 4., 8.),
            glm::dvec3(4., 8., 8.),
            glm::dvec3(4., 8., 4.),
            glm::dvec3(8., 8., 4.),
            glm::dvec3(8., 4., 4.),
            glm::dvec3(8., 4., 8.),
            ].into_iter().cycle();
        for i in -16..16 {
            rscene.insert_object(
                SceneObject::new_cylinder(
                    colors.next().unwrap(),
                    BASE_EMISSION,
                    SceneObjectMaterial::Diffuse,
                    Ray::new(
                        glm::dvec3(0., i as f64 / 8., -4.),
                        glm::dvec3(1., 0., 0.)),
                    2.5,
                    0.125,
                    CylinderType::DoubleCap
                ).unwrap()
            )
        }

        rscene
    }
}