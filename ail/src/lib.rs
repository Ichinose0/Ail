use std::{fmt::Debug, sync::Mutex};

use aom::{Object, ID};
use management::{RenderManager, WidgetRegistry};
use widget::{Button, Widget};
use winit::event_loop::EventLoop;

pub mod event;
pub mod management;
pub mod render;
pub mod widget;

pub type CursorIcon = winit::window::CursorIcon;

/// Represents an area on the screen
///
/// This structure represents an area (rectangle) by the coordinates of the upper left and lower right corners
/// # Members
/// * `left` - X coordinate of upper left corner
/// * `top` - Y coordinate of upper left corner
/// * `right` - X coordinate of lower right corner
/// * `bottom` - Y coordinate of lower right corner
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Rect {
    /// Obtains the X coordinate
    pub fn x(&self) -> u32 {
        self.left
    }

    /// Obtains the Y coordinate
    pub fn y(&self) -> u32 {
        self.top
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }

    pub fn from_coordinate(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            left: x,
            top: y,
            right: x + width,
            bottom: y + height,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::from_coordinate(0, 0, 800, 600)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ApplicationEvent<M>
where
    M: Clone + Copy + Debug,
{
    Close,
    RedrawRequested,
    Message(M),
}

pub struct Window {
    event_loop: Option<EventLoop<()>>,
    inner: winit::window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        let inner = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        Self {
            event_loop: Some(event_loop),
            inner,
        }
    }
}

pub struct Application<W, M>
where
    W: Widget<M> + 'static,
    M: Clone + Copy + Debug,
{
    window: Window,
    widget: W,
    render_manager: RenderManager<M>,
}

impl<W, M> Application<W, M>
where
    W: Widget<M> + 'static,
    M: Clone + Copy + Debug,
{
    pub fn new(window: Window, widget: W) -> Self {
        let render_manager = RenderManager::new(&window.inner);
        Self {
            window,
            widget,
            render_manager,
        }
    }

    pub fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(ApplicationEvent<M>,&mut WidgetRegistry<M>),
    {
        let id = self.widget.id();
        self.render_manager.register(self.widget);
        let mut ids = vec![id];
        
        self.window
            .event_loop
            .unwrap()
            .run(|e, elwt| match e {
                winit::event::Event::NewEvents(e) => {}
                winit::event::Event::WindowEvent { window_id, event } => match event {
                    winit::event::WindowEvent::RedrawRequested => {
                        self.render_manager.render(&[id]);
                    }

                    winit::event::WindowEvent::Resized(size) => {
                        self.render_manager.resize(size.width, size.height);
                    }

                    winit::event::WindowEvent::CloseRequested => {
                        elwt.exit();
                    }

                    winit::event::WindowEvent::CursorMoved {
                        device_id,
                        position,
                    } => {
                        self.window.inner.request_redraw();
                        let s = Mutex::new(5);
                        let mut events = vec![];
                        for i in &ids {
                            let mut registry = self.render_manager.registry.lock().unwrap();
                            let widget = registry.search_mut(id);
                            for area in widget.area() {
                                let x = position.x as i32;
                                let y = position.y as i32;
                                let cx = (area.left) as i32;
                                let cy = (area.top) as i32;
                                let width = (area.right - area.left) as i32;
                                let height = (area.bottom - area.top) as i32;
                                if x >= cx && x <= cx + width {
                                    if y >= cy && y <= cy + height {
                                        match widget.on_hover() {
                                            Some(m) => {events.push((*i,m))},
                                            None => {}
                                        }
                                        // self.window.inner.set_cursor_icon(comp.cursor());
                                        // request_redraw = true;
                                        // if get_key_state(VK_LBUTTON) {
                                        //     comp.message(widget::WidgetMessage::OnClick, data);
                                        //     request_redraw = true;
                                        //     std::thread::sleep(Duration::from_millis(200));
                                        // }
                                    } else {
                                        //comp.message(widget::WidgetMessage::Unfocus, data);
                                    }
                                } else {
                                    //comp.message(widget::WidgetMessage::Unfocus, data);
                                }
                            }
                        }

                        let mut registry = self.render_manager.registry.lock().unwrap();

                        for (id,mes) in events {
                            callback(ApplicationEvent::Message(mes),&mut registry);
                        }
                    }

                    _ => {}
                },
                winit::event::Event::Resumed => {}
            
                winit::event::Event::MemoryWarning => {}

                _ => {}
            })
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
