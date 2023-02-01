use crate::{
    scene::{
        Scene,
        obj::{SceneObject, SceneObjectMaterial, CylinderType}, sample::SampleScene
    },
    common::Ray
};

pub struct ThreeCylindersWithLightsSampleScene;

impl SampleScene for ThreeCylindersWithLightsSampleScene {
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
        // Cylinders + Light 1
        rscene.insert_object(
            SceneObject::new_cylinder(
                nalgebra_glm::DVec3::new(8., 4., 4.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                Ray::new(
                    nalgebra_glm::DVec3::new(0., 0., -5.0),
                    nalgebra_glm::DVec3::new(1., 0., 0.),
                ),
                1.25,
                0.5,
                CylinderType::ThroughHole
            ).expect("This cylinder should respects all constraints.")
        );
        rscene.insert_object(
            SceneObject::new_sphere(
                nalgebra_glm::DVec3::new(0., 0., 0.),
                LIGHT_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(0., 0., -5.0),
                0.375
            )
        );
        // Cylinders + Light 2
        rscene.insert_object(
            SceneObject::new_cylinder(
                nalgebra_glm::DVec3::new(4., 8., 4.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                Ray::new(
                    nalgebra_glm::DVec3::new(0., -2.25, -4.0),
                    nalgebra_glm::DVec3::new(1., 0., 0.),
                ),
                1.25,
                0.5,
                CylinderType::SingleCap
            ).expect("This cylinder should respects all constraints.")
        );
        rscene.insert_object(
            SceneObject::new_sphere(
                nalgebra_glm::DVec3::new(0., 0., 0.),
                LIGHT_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(0., -2.25, -4.0),
                0.375
            )
        );
        // Cylinders + Light 3
        rscene.insert_object(
            SceneObject::new_cylinder(
                nalgebra_glm::DVec3::new(4., 4., 8.),
                BASE_EMISSION,
                SceneObjectMaterial::Diffuse,
                Ray::new(
                    nalgebra_glm::DVec3::new(0., 2.25, -4.0),
                    nalgebra_glm::DVec3::new(1., 0., 0.),
                ),
                1.25,
                0.5,
                CylinderType::DoubleCap
            ).expect("This cylinder should respects all constraints.")
        );
        rscene.insert_object(
            SceneObject::new_sphere(
                nalgebra_glm::DVec3::new(0., 0., 0.),
                LIGHT_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(0., 2.25, -4.0),
                0.375
            )
        );
        // Light
        rscene.insert_object(
            SceneObject::new_sphere(
                nalgebra_glm::DVec3::new(0., 0., 0.),
                LIGHT_EMISSION,
                SceneObjectMaterial::Diffuse,
                nalgebra_glm::DVec3::new(-1.9, 0., -3.),
                0.5
            )
        );

        rscene
    }
}