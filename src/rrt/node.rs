use glam::Vec2;

pub struct Node {
    id: u32,
    parent_id: Option<u32>,
    pub children: Vec<u32>,
    pos: Vec2,
}

static mut ID: u32 = 0;

impl Node {
    pub fn new(parent_id: Option<u32>, pos: Vec2) -> Self {
        let id = unsafe {
            let id = ID;
            ID += 1;
            id
        };
        Self {
            id,
            parent_id,
            children: vec![],
            pos,
        }
    }

    pub fn get_pos_arr(&self) -> [f64; 2] {
        self.pos.to_array().map(|x| x as f64)
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn add_child(&mut self, id: u32) {
        self.children.push(id)
    }
}
