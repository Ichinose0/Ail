use std::fmt::Debug;

use aom::{ID, Object};
use management::{WidgetRegistry, RenderManager};
use widget::{Button, Widget};
use winit::event_loop::EventLoop;

pub mod management;
pub mod widget;
pub mod event;
pub mod render;

pub type CursorIcon = winit::window::CursorIcon;

#[derive(Clone,Copy,Debug)]
pub enum ApplicationEvent<M> 
where
    M: Clone + Copy + Debug
{
    Close,
    RedrawRequested,
    Message(M)
}

pub struct Window {
    event_loop: Option<EventLoop<()>>,
    inner: winit::window::Window
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        let inner = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
        Self {
            event_loop: Some(event_loop),
            inner
        }
    }
}

pub struct Application<W,M> 
where
    W: Widget<M> + 'static,
    M: Clone + Copy + Debug
{
    window: Window,
    widget: W,
    render_manager: RenderManager<M>
}

impl<W,M> Application<W,M> 
where
    W: Widget<M> + 'static,
    M: Clone + Copy + Debug
{
    pub fn new(window: Window,widget: W) -> Self {
        let render_manager = RenderManager::new(&window.inner);
        Self {
            window,
            widget,
            render_manager
        }
    }

    pub fn run<F>(mut self,callback: F)
    where
        F: FnMut(ApplicationEvent<M>)
    {
        let id = self.widget.id();
        self.render_manager.register(self.widget);
        self.window.event_loop.unwrap().run(|e,elwt| {
            match e {
                winit::event::Event::NewEvents(e) => {}
                winit::event::Event::WindowEvent { window_id, event } => {
                    match event {
                        winit::event::WindowEvent::RedrawRequested => {
                            self.render_manager.render(&[id]);
                        },

                        winit::event::WindowEvent::Resized(size) => {
                            self.render_manager.resize(size.width,size.height);
                        }

                        winit::event::WindowEvent::CloseRequested => {
                            elwt.exit();
                        }

                        _ => {}
                    }
                },
                winit::event::Event::Resumed => {},
                winit::event::Event::MemoryWarning => {},

                _ => {}
            }
        }).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
