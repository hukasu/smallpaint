use criterion::{Criterion, BenchmarkId};
use criterion::{criterion_main, criterion_group};
use smallpaint::Scene;
use smallpaint::camera::SimpleCamera;
use smallpaint::renderer::Renderer;
use smallpaint::sampler::RandomSampler;
use smallpaint::scene::obj::{SceneObject, SceneObjectMaterial};
use smallpaint::scene::storage::{SceneObjectStorage, BoundingVolumeHierarchy};
use smallpaint::terminator::DepthTerminator;
use smallpaint::tracer::SimpleTracer;

fn build_vec_storage(item_count: usize) -> Vec<SceneObject> {
    const BASE_EMISSION: f64 = 0.;
    let mut v = vec![];// Room walls
    v.insert_object(
        SceneObject::new_plane(
            nalgebra_glm::DVec3::new(6., 6., 6.),
            BASE_EMISSION,
            SceneObjectMaterial::Diffuse,
            nalgebra_glm::DVec3::new(-3000., 0., 0.),
            nalgebra_glm::DVec3::new(1., 0., 0.)
        )
    );
    v.insert_object(
        SceneObject::new_plane(
            nalgebra_glm::DVec3::new(6., 6., 6.),
            BASE_EMISSION,
            SceneObjectMaterial::Diffuse,
            nalgebra_glm::DVec3::new(2000.5, 0., 0.),
            nalgebra_glm::DVec3::new(-1., 0., 0.)
        )
    );
    v.insert_object(
        SceneObject::new_plane(
            nalgebra_glm::DVec3::new(10., 2., 2.),
            BASE_EMISSION,
            SceneObjectMaterial::Diffuse,
            nalgebra_glm::DVec3::new(0., -2000.75, 0.),
            nalgebra_glm::DVec3::new(0., 1., 0.)
        )
    );
    v.insert_object(
        SceneObject::new_plane(
            nalgebra_glm::DVec3::new(2., 10., 2.),
            BASE_EMISSION,
            SceneObjectMaterial::Diffuse,
            nalgebra_glm::DVec3::new(0., 2000.75, 0.),
            nalgebra_glm::DVec3::new(0., -1., 0.)
        )
    );
    v.insert_object(
        SceneObject::new_plane(
            nalgebra_glm::DVec3::new(6., 6., 6.),
            BASE_EMISSION,
            SceneObjectMaterial::Diffuse,
            nalgebra_glm::DVec3::new(0., 0., -5000.5),
            nalgebra_glm::DVec3::new(0., 0., 1.)
        )
    );
    v.insert_object(
        SceneObject::new_plane(
            nalgebra_glm::DVec3::new(6., 6., 6.),
            BASE_EMISSION,
            SceneObjectMaterial::Diffuse,
            nalgebra_glm::DVec3::new(0., 0., 1000.5),
            nalgebra_glm::DVec3::new(0., 0., -1.)
        )
    );
    for x in 0..item_count {
        for y in 0..item_count {
            for z in 0..item_count {
                v.insert_object(
                    SceneObject::new_sphere(
                        nalgebra_glm::DVec3::from_element(1.),
                        BASE_EMISSION,
                        SceneObjectMaterial::Diffuse,
                        nalgebra_glm::DVec3::new(x as f64, y as f64, z as f64),
                        0.5
                    )
                );
            }
        }
    }
    v
}

pub fn criterion_benchmark(c: &mut Criterion) {
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;
    const SAMPLES_PER_PIXEL: u64 = 25;
    const REFRACTION_INDEX: f64 = 1.5;
    const MAX_DEPTH: usize = 20;

    let mut group = c.benchmark_group("Storage Bench");

    let item_counts = [1_usize, 2, 3, 4, 5];

    item_counts.iter().for_each(
        |item_count| {
            let id = BenchmarkId::new("Vector Storage", item_count);
            group.bench_with_input(id, item_count, |b, item_count| {
                let tracer = SimpleTracer::new(
                    Box::new(DepthTerminator::new(MAX_DEPTH)),
                    Box::new(RandomSampler::new())
                );
            
                let camera = SimpleCamera::new(WIDTH as f64, HEIGHT as f64);

                let v_storage = build_vec_storage(*item_count);
                let scene = Scene::new(Box::new(v_storage));

                b.iter(
                    || {
                        let mut renderer: Renderer = Renderer::new(
                            WIDTH,
                            HEIGHT,
                            REFRACTION_INDEX,
                            SAMPLES_PER_PIXEL
                        );
                        
                        renderer.render(&tracer, &camera, &scene)
                    }
                )
            });
        }
    );
    item_counts.iter().for_each(
        |item_count| {
            let id = BenchmarkId::new("BVH Storage", item_count);
            group.bench_with_input(id, item_count, |b, item_count| {
                let tracer = SimpleTracer::new(
                    Box::new(DepthTerminator::new(MAX_DEPTH)),
                    Box::new(RandomSampler::new())
                );
            
                let camera = SimpleCamera::new(WIDTH as f64, HEIGHT as f64);

                let v_storage = build_vec_storage(*item_count);
                let mut bvh_storage = BoundingVolumeHierarchy::from(v_storage);
                bvh_storage.rebuild(1);
                let scene = Scene::new(Box::new(bvh_storage));

                b.iter(
                    || {
                        let mut renderer: Renderer = Renderer::new(
                            WIDTH,
                            HEIGHT,
                            REFRACTION_INDEX,
                            SAMPLES_PER_PIXEL
                        );

                        renderer.render(&tracer, &camera, &scene)
                    }
                )
            });
        }
    );
    
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);