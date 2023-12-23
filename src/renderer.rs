use std::thread;
use tracing::{debug, info};

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }
    pub fn render(&mut self) {
        thread::scope(|s| {
            let thread_cnt = num_cpus::get();
            info!("Worker threads: {thread_cnt}");
            for i in 0..thread_cnt {
                let mut worker = Worker {
                    id: i,
                    renderer: &self,
                };
                s.spawn(move || worker.run());
            }
        })
    }
}

struct Worker<'a> {
    id: usize,
    renderer: &'a Renderer,
}

impl<'a> Worker<'a> {
    fn run(&mut self) {
        debug!("Worker started (id: {})", self.id);
    }
}
