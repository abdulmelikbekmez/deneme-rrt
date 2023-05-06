use std::collections::HashMap;

use eframe::egui::plot::{Line, PlotPoint, PlotPoints, Points};
use glam::{vec2, Vec2};
use kdtree::{distance::squared_euclidean, KdTree};

use super::node::Node;

pub struct RRTInner {
    nodes: HashMap<u32, Node>,
    kdtree: KdTree<f64, u32, [f64; 2]>,
    goal: Vec2,
}

impl Into<Points> for &RRTInner {
    fn into(self) -> Points {
        Points::new(
            self.nodes
                .iter()
                .map(|(_, node)| node.get_pos_arr())
                .collect::<Vec<[f64; 2]>>(),
        )
    }
}

impl RRTInner {
    pub fn new(start: &PlotPoint, finish: &PlotPoint) -> Self {
        let mut map = HashMap::new();
        let start_node = Node::new(None, vec2(start.x as f32, start.y as f32));

        let mut kdtree = KdTree::new(2);
        kdtree
            .add(start_node.get_pos_arr(), start_node.get_id())
            .unwrap();
        map.insert(start_node.get_id(), start_node);
        Self {
            nodes: map,
            kdtree,
            goal: vec2(finish.x as f32, finish.y as f32),
        }
    }

    pub fn lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        for (_, node) in self.nodes.iter() {
            for child_id in node.children.iter() {
                let child_node = self.nodes.get(child_id).unwrap();
                lines.push(Line::new(PlotPoints::new(vec![
                    node.get_pos_arr(),
                    child_node.get_pos_arr(),
                ])));
            }
        }
        lines
    }

    pub fn add_node(&mut self, parent_id: Option<u32>, pos: Vec2) {
        let node = Node::new(parent_id, pos);
        let node_id = node.get_id();
        if let Some(parent_id) = parent_id {
            self.nodes
                .get_mut(&parent_id)
                .expect("There is no node with given parent id")
                .add_child(node_id);
        }
        self.kdtree.add(node.get_pos_arr(), node.get_id()).unwrap();
        self.nodes.insert(node_id, node);
    }

    pub fn get_closest_node(&self, pos: Vec2) -> Option<(u32, Vec2)> {
        let x = pos.to_array().map(|x| x as f64);
        let (_, &id) = self
            .kdtree
            .nearest(&x, 1, &squared_euclidean)
            .ok()?
            .first()?;

        self.nodes
            .get(&id)
            .map(|node| (node.get_id(), node.get_pos()))
    }
}
