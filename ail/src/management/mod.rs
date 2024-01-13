use std::collections::HashMap;

use aom::ID;
use winit::window::Window;

use crate::{widget::Widget, render::Renderer};

pub struct RenderManager {
    registry: WidgetRegistry,
    renderer: Renderer
}

impl RenderManager {
    pub fn new(window: &Window) -> Self {
        let registry = WidgetRegistry::new();
        let renderer = Renderer::new(window);
        Self {
            registry,
            renderer
        }
    }

    pub fn register<W>(&mut self,widget: W)
    where
        W: Widget + 'static
    {
        self.registry.register(widget);
    }

    pub fn render(&mut self,id: &[ID]) {
        self.renderer.begin();
        for i in id {
            let w = self.registry.search_mut(*i);
            self.renderer.render(w);
        }
        self.renderer.end();
    }
}

pub struct WidgetRegistry {
    map: HashMap<ID,Box<dyn Widget>>
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub(crate) fn register<W>(&mut self,widget: W) 
    where
        W: Widget + 'static
    {
        let id = widget.id();
        self.map.insert(id, Box::new(widget));
    }

    pub fn search(&self,id: ID) -> &Box<dyn Widget> {
        &self.map[&id]
    }

    pub fn search_mut(&mut self,id: ID) -> &mut Box<dyn Widget> {
        self.map.get_mut(&id).unwrap()
    }
}