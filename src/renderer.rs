use crate::ppm::Image;
use crate::scene::{
    AbsoluteSphereScene, Camera, DemoSkyScene, NormVectorVisualizedSphereScene, Scene,
};
use crate::types::{NumPosition, Pixel, PositionVec};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use tracing::{debug, info};

pub struct Renderer<T: Scene> {
    camera: Camera,
    scene: T,
}

impl<T: Scene> Renderer<T> {
    pub fn render(&self, mut samples: usize) {
        let (sender, receiver) = channel::<Image>();
        thread::scope(move |s| {
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
            s.spawn(move || {
                // TODO add SSAA, we just keep the first image and ignore all others for now
                let image = receiver.recv().expect("expecting at least one image");
                for _ in receiver {}
                image
                    .save(Path::new("result.ppm"))
                    .expect("failed to save image file");
            });
        });
    }
}

pub fn new_demo_renderer() -> Renderer<DemoSkyScene> {
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
        scene: DemoSkyScene {},
    }
}

pub fn new_sphere_renderer() -> Renderer<AbsoluteSphereScene> {
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
        scene: AbsoluteSphereScene {
            sphere_center: PositionVec::new(0.0, 0.0, -1.0),
            sphere_radius: 0.5,
            sphere_color: Pixel::black(),
        },
    }
}

pub fn new_norm_visualized_sphere_renderer() -> Renderer<NormVectorVisualizedSphereScene> {
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
        scene: NormVectorVisualizedSphereScene {
            sphere_center: PositionVec::new(0.0, 0.0, -1.0),
            sphere_radius: 0.5,
        },
    }
}

struct Worker<'a, T: Scene + Send + Sync> {
    id: usize,
    renderer: &'a Renderer<T>,
    ch: Sender<Image>,
    iter_count: usize,
}

impl<'a, T: Scene> Worker<'a, T> {
    fn run(&self) {
        debug!(
            "Worker started (id: {}), iter_count: {}",
            self.id, self.iter_count
        );
        for _ in 0..self.iter_count {
            let scene = &self.renderer.scene;
            let camera = &self.renderer.camera;
            // TODO add SSAA
            let image = camera.get_image(scene, 0.0, 0.0);
            self.ch
                .send(image)
                .expect("failed to write worker result image to channel");
        }
    }
}
