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

    pub fn render<W>(&mut self,widgets: &[&Box<W>]) 
    where
        W: Widget
    {
        self.renderer.render(widgets);
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