use super::prelude::*;

#[derive(Resource, Default)]
pub struct NpcCount {
    pub total: usize,
    pub visible: usize,
}

impl NpcCount {
    pub fn reset(&mut self) {
        self.total = 0;
        self.visible = 0;
    }
}
