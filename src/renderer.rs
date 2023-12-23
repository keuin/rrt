use crate::ppm::Image;
use crate::scene::{Camera, DemoSkyScene, Scene};
use crate::types::{NumPosition, PositionVec};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use tracing::{debug, info};

pub struct Renderer<T: Scene> {
    camera: Camera,
    scene: T,
}

impl Renderer<DemoSkyScene> {
    pub fn new() -> Self {
        Renderer {
            camera: Camera {
                pos: PositionVec::zero(),
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
    pub fn render(&mut self) {
        thread::scope(|s| {
            let thread_cnt = num_cpus::get();
            info!("Worker threads: {thread_cnt}");
            let (sender, receiver) = channel::<Image>();
            for i in 0..thread_cnt {
                let worker = Worker {
                    id: i,
                    renderer: &self,
                    ch: sender.clone(),
                    iter_count: 0,
                };
                s.spawn(move || worker.run());
            }
            drop(sender);
            // TODO add SSAA, we just keep the first image and ignore all others for now
            let image = receiver.recv().expect("expecting at least one image");
            for _ in receiver {}
            image
                .save(Path::new("result.ppm"))
                .expect("failed to save image file");
        })
    }
}

struct Worker<'a, T: Scene> {
    id: usize,
    renderer: &'a Renderer<T>,
    ch: Sender<Image>,
    iter_count: usize,
}

impl<'a, T: Scene> Worker<'a, T> {
    fn run(&self) {
        debug!("Worker started (id: {})", self.id);
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
