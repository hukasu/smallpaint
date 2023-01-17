use smallpaint::{
    renderer::Renderer,
    sampler::RandomSampler,
    scene::sample::{SampleScene, ThreeSpheresSampleScene},
    tracer::FresnelTracer,
    camera::SimpleCamera,
    terminator::RussianRouletteTerminator,
    writer::{Writer, ppm::PPMWriter}
};

fn main() {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;
    const SAMPLES_PER_PIXEL: u64 = 25;
    const REFRACTION_INDEX: f64 = 1.5;
    const ROULETTE_DEPTH: usize = 5;
    const ROULETTE_PROB: f64 = 0.1;

    let tracer = FresnelTracer::new(
        Box::new(RussianRouletteTerminator::new(ROULETTE_DEPTH, ROULETTE_PROB)),
        Box::new(RandomSampler::new())
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
    PPMWriter::write(&renderer, "./fresnel_example.ppm");
}