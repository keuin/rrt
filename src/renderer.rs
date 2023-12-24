use crate::ppm::Image;
use crate::scene::{
    AbsoluteSphereScene, Camera, DemoSkyScene, Hittable, NormVectorVisualizedSphereScene, Scene,
    SkiedWorld,
};
use crate::types::{NumPosition, Pixel, PositionVec};
use rand::Rng;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use tracing::{debug, info};

pub struct Renderer<T>
where
    T: Scene,
{
    camera: Camera,
    scene: T,
}

impl<T: Scene> Renderer<T> {
    pub fn render(&self, samples: usize) {
        let (sender, receiver) = channel::<Image<T::T>>();
        thread::scope(move |s| {
            {
                let mut samples = samples;
                let thread_cnt = num_cpus::get();
                info!("Worker threads: {thread_cnt}");
                let samples_per_thread = samples / thread_cnt;
                for i in 0..thread_cnt {
                    let worker = Worker {
                        id: i,
                        renderer: &self,
                        ch: sender.clone(),
                        iter_count: if i == thread_cnt - 1 {
                            samples_per_thread + samples % thread_cnt
                        } else {
                            samples_per_thread
                        },
                    };
                    s.spawn(move || worker.run());
                    samples -= samples_per_thread;
                }
            }
            s.spawn(move || {
                // TODO add SSAA, we just keep the first image and ignore all others for now
                let sample_factor = 1.0 / samples as f64;
                let mut sum_image: Image<T::T> = Image::new(self.camera.width, self.camera.height);
                let mut has_image = false;
                for mut image in receiver {
                    image *= sample_factor;
                    sum_image += image;
                    has_image = true;
                }
                if !has_image {
                    panic!("no image generated");
                }
                sum_image
                    .save(Path::new("result.ppm"))
                    .expect("failed to save image file");
            });
        });
    }
}

pub fn new_demo_renderer<T: Pixel>() -> Renderer<DemoSkyScene<T>> {
    Renderer {
        camera: Camera {
            pos: PositionVec::zeros(),
            // wh_ratio: 0.0,
            width: 640,
            height: 480,
            pixel_width: 0.125,
            pixel_height: 0.125,
            focus_length: 1 as NumPosition,
        },
        scene: DemoSkyScene::new(),
    }
}

pub fn new_sphere_renderer<T: Pixel>() -> Renderer<AbsoluteSphereScene<T>> {
    Renderer {
        camera: Camera {
            pos: PositionVec::zeros(),
            // wh_ratio: 0.0,
            width: 640,
            height: 480,
            pixel_width: 1.0 / 256.0,
            pixel_height: 1.0 / 256.0,
            focus_length: 1 as NumPosition,
        },
        scene: AbsoluteSphereScene::new(PositionVec::new(0.0, 0.0, -1.0), 0.5, T::black()),
    }
}

pub fn new_norm_visualized_sphere_renderer<T: Pixel>(
) -> Renderer<NormVectorVisualizedSphereScene<T>> {
    Renderer {
        camera: Camera {
            pos: PositionVec::zeros(),
            // wh_ratio: 0.0,
            width: 640,
            height: 480,
            pixel_width: 1.0 / 256.0,
            pixel_height: 1.0 / 256.0,
            focus_length: 1 as NumPosition,
        },
        scene: NormVectorVisualizedSphereScene::new(PositionVec::new(0.0, 0.0, -1.0), 0.5),
    }
}

pub fn new_skied_world<'a, T: Pixel>(
    objects: Vec<&'a dyn Hittable<T>>,
) -> Renderer<SkiedWorld<'a, T>> {
    Renderer {
        camera: Camera {
            pos: PositionVec::zeros(),
            // wh_ratio: 0.0,
            width: 640,
            height: 480,
            pixel_width: 1.0 / 256.0,
            pixel_height: 1.0 / 256.0,
            focus_length: 1 as NumPosition,
        },
        scene: SkiedWorld { objects },
    }
}

struct Worker<'a, T: Scene + Send + Sync> {
    id: usize,
    renderer: &'a Renderer<T>,
    ch: Sender<Image<T::T>>,
    iter_count: usize,
}

impl<'a, T: Scene> Worker<'a, T> {
    fn run(&self) {
        debug!(
            "Worker started (id: {}), iter_count: {}",
            self.id, self.iter_count
        );
        // TODO make SSAA image generation deterministic
        let mut rng = rand::thread_rng();
        for _ in 0..self.iter_count {
            let scene = &self.renderer.scene;
            let camera = &self.renderer.camera;
            // TODO add SSAA
            let rnd_x: f64 = rng.gen();
            let rnd_y: f64 = rng.gen();
            let image = camera.get_image(scene, rnd_x, rnd_y);
            self.ch
                .send(image)
                .expect("failed to write worker result image to channel");
        }
    }
}
