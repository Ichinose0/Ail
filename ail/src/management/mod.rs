use std::collections::HashMap;

use aom::ID;

use crate::widget::Widget;

pub struct WidgetRegistry {
    map: HashMap<ID,Box<dyn Widget>>
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn register<W>(&mut self,widget: W) 
    where
        W: Widget + 'static
    {
        let id = widget.id();
        self.map.insert(id, Box::new(widget));
    }

    pub fn search(&self,id: ID) -> &Box<dyn Widget> {
        &self.map[&id]
    }
}