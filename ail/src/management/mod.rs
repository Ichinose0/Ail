use std::{collections::HashMap, fmt::Debug};

use aom::ID;
use winit::window::Window;

use crate::{widget::Widget, render::Renderer};

pub struct RenderManager<M> 
where
    M: Clone + Copy + Debug
{
    pub(crate) registry: WidgetRegistry<M>,
    renderer: Renderer
}

impl<M> RenderManager<M> 
where
    M: Clone + Copy + Debug
{
    pub fn new(window: &Window) -> Self {
        let registry = WidgetRegistry::new();
        let renderer = Renderer::new(window);
        Self {
            registry,
            renderer
        }
    }

    pub fn resize(&mut self,width: u32,height: u32) {
        self.renderer.resize(width,height);
    }

    pub fn register<W>(&mut self,widget: W)
    where
        W: Widget<M> + 'static
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

pub struct WidgetRegistry<M> 
where
    M: Clone + Copy + Debug
{
    map: HashMap<ID,Box<dyn Widget<M>>>
}

impl<M> WidgetRegistry<M> 
where
    M: Clone + Copy + Debug
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn get<T>(&self,id: ID) -> Option<&Box<T>> {
        match self.map.get(&id) {
            Some(s) => return Some(unsafe { std::mem::transmute(s)} ),
            None => return None,
        }
    }

    pub fn get_mut<T>(&mut self,id: ID) -> Option<&mut Box<T>> {
        match self.map.get_mut(&id) {
            Some(s) => return Some(unsafe { std::mem::transmute(s)} ),
            None => return None,
        }
    }

    pub(crate) fn register<W>(&mut self,widget: W) 
    where
        W: Widget<M> + 'static
    {
        let id = widget.id();
        self.map.insert(id, Box::new(widget));
    }

    pub(crate) fn search(&self,id: ID) -> &Box<dyn Widget<M>> {
        &self.map[&id]
    }

    pub(crate) fn search_mut(&mut self,id: ID) -> &mut Box<dyn Widget<M>> {
        self.map.get_mut(&id).unwrap()
    }
}