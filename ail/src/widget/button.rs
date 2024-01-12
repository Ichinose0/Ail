use aom::{ID,Object};

use super::Widget;

#[derive(Debug)]
pub struct Button {
    id: ID
}

impl Button {
    pub fn new(id: &'static str) -> Self {
        Self {
            id: ID::from(id)
        }
    }
}

impl Widget for Button {
    
}

impl Object for Button {
    fn id(&self) -> ID {
        self.id
    }
}