use smallpaint::{
    renderer::Renderer,
    sampler::HaltonSampler,
    scene::{Scene, obj::{SceneObjectMaterial, SceneObject}},
    tracer::PainterlyTracer,
    camera::SimpleCamera,
    terminator::DepthTerminator,
    writer::{Writer, ppm::PPMWriter}
};

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;
    const SAMPLES_PER_PIXEL: u64 = 25;
    const REFRACTION_INDEX: f64 = 1.5;
    const MAX_DEPTH: usize = 20;
    const BASE_EMISSION: f64 = 0.;
    const LIGHT_EMISSION: f64 = 5_000.;

    let tracer = PainterlyTracer::new(
        Box::new(DepthTerminator::new(MAX_DEPTH)),
        Box::new(HaltonSampler::new())
    );
    let mut renderer: Renderer = Renderer::new(
        WIDTH,
        HEIGHT,
        REFRACTION_INDEX,
        SAMPLES_PER_PIXEL
    );

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

    let camera = SimpleCamera::new(WIDTH as f64, HEIGHT as f64);
    
    renderer.render(&tracer, &camera, &rscene).unwrap();
    PPMWriter::write(&renderer, "./painterly_example.ppm");
}