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
pub enum ApplicationEvent {
    Close,
    SetCursor(CursorIcon)
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

pub struct Application {
    window: Window,
    render_manager: RenderManager
}

impl Application {
    pub fn new(window: Window) -> Self {
        let render_manager = RenderManager::new(&window.inner);
        Self {
            window,
            render_manager
        }
    }

    pub fn run(mut self)
    {
        let button = Box::new(Button::new("sample"));
        let widgets = &[&button];

        self.window.event_loop.unwrap().run(|e,_| {
            match e {
                winit::event::Event::NewEvents(e) => {}
                winit::event::Event::WindowEvent { window_id, event } => {
                    match event {
                        winit::event::WindowEvent::RedrawRequested => {
                            self.render_manager.render(widgets);
                        },

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

pub fn run() {
    let mut registry = WidgetRegistry::new();
    let button = Button::new("button");
    let origin_id = button.id();
    registry.register(button);
    let widget = registry.search(ID::from("button"));
    let id = widget.id();
    assert_eq!(origin_id,id);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
