mod node;
mod tree;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use eframe::egui::plot::{PlotPoint, PlotUi};
use glam::{vec2, Vec2};
use rand::{distributions::Uniform, prelude::Distribution};

use self::tree::RRTInner;

pub struct RRT {
    tree: Arc<Mutex<RRTInner>>,
    finish: Vec2,
}
const MIN: f32 = -10.0;
const MAX: f32 = 10.0;

impl RRT {
    pub fn new(start: &PlotPoint, finish: &PlotPoint) -> Self {
        Self {
            tree: Arc::new(Mutex::new(RRTInner::new(start, finish))),
            finish: vec2(finish.x as f32, finish.y as f32),
        }
    }

    pub fn draw(&self, ui: &mut PlotUi) {
        let tree = &*self.tree.lock().unwrap();
        ui.points(tree.into());
        tree.lines().into_iter().for_each(|line| {
            ui.line(line);
        })
    }

    fn random_point() -> Vec2 {
        let uniform = Uniform::new::<f32, f32>(MIN, MAX);
        let mut rng = rand::thread_rng();
        vec2(uniform.sample(&mut rng), uniform.sample(&mut rng))
    }

    pub fn start(&mut self) {
        let tree = Arc::clone(&self.tree);
        let finish = self.finish;
        std::thread::spawn(move || loop {
            {
                let mut random_point = Self::random_point();
                let mut tree = tree.lock().unwrap();
                let (node_id, node_pos) = tree.get_closest_node(random_point).unwrap();
                let dif = random_point - node_pos;
                if dif.length() > 0.2 {
                    random_point = node_pos + dif.normalize() * 0.2;
                    tree.add_node(Some(node_id), random_point);
                }
                if (random_point - finish).length() < 0.2 {
                    break;
                }
            }
            std::thread::sleep(Duration::from_nanos(1));
        });
    }
}
