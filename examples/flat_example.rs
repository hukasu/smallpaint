use smallpaint::{
    renderer::Renderer,
    scene::sample::{SampleScene, ThreeSpheresSampleScene},
    tracer::{FlatTracer},
    camera::SimpleCamera,
    writer::{Writer, ppm::PPMWriter}
};

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;
    const SAMPLES_PER_PIXEL: u64 = 1;
    const REFRACTION_INDEX: f64 = 1.5;

    let tracer = FlatTracer::new();
    let mut renderer: Renderer = Renderer::new(
        WIDTH,
        HEIGHT,
        REFRACTION_INDEX,
        SAMPLES_PER_PIXEL
    );

    let scene = ThreeSpheresSampleScene::build_sample_scene();

    let camera = SimpleCamera::new(WIDTH as f64, HEIGHT as f64);
    
    renderer.render(&tracer, &camera, &scene).unwrap();
    PPMWriter::write(&renderer, "./flat_example.ppm");
}