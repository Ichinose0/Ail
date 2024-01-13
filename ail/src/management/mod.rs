use std::{collections::HashMap, fmt::Debug, sync::Mutex};

use aom::ID;
use winit::window::Window;

use crate::{render::Renderer, widget::Widget};

pub struct RenderManager {
    pub(crate) registry: Mutex<WidgetRegistry>,
    renderer: Renderer,
}

impl RenderManager {
    pub fn new(window: &Window) -> Self {
        let registry = Mutex::new(WidgetRegistry::new());
        let renderer = Renderer::new(window);
        Self { registry, renderer }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn register<W>(&mut self, widget: W)
    where
        W: Widget + 'static,
    {
        self.registry.lock().unwrap().register(widget);
    }

    pub fn render(&mut self, id: &[ID]) {
        self.renderer.begin();
        let mut registry = self.registry.lock().unwrap();
        for i in id {
            let w = registry.search_mut(*i);
            self.renderer.render(w);
        }
        self.renderer.end();
    }
}

pub struct WidgetRegistry {
    map: HashMap<ID, Box<dyn Widget>>,
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get<T>(&self, id: ID) -> Option<&Box<T>> {
        match self.map.get(&id) {
            Some(s) => return Some(unsafe { std::mem::transmute(s) }),
            None => return None,
        }
    }

    pub fn get_mut<T>(&mut self, id: ID) -> Option<&mut Box<T>> {
        match self.map.get_mut(&id) {
            Some(s) => return Some(unsafe { std::mem::transmute(s) }),
            None => return None,
        }
    }

    pub(crate) fn register<W>(&mut self, widget: W)
    where
        W: Widget + 'static,
    {
        let id = widget.id();
        self.map.insert(id, Box::new(widget));
    }

    pub(crate) fn search(&self, id: &ID) -> &Box<dyn Widget> {
        &self.map[id]
    }

    pub(crate) fn search_mut(&mut self, id: ID) -> &mut Box<dyn Widget> {
        self.map.get_mut(&id).unwrap()
    }
}
