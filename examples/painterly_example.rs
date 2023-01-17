use smallpaint::{
    renderer::Renderer,
    sampler::HaltonSampler,
    scene::sample::{SampleScene, ThreeSpheresSampleScene},
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

    let scene = ThreeSpheresSampleScene::build_sample_scene();

    let camera = SimpleCamera::new(WIDTH as f64, HEIGHT as f64);
    
    renderer.render(&tracer, &camera, &scene).unwrap();
    PPMWriter::write(&renderer, "./painterly_example.ppm");
}