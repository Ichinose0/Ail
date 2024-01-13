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
pub type WindowTheme = winit::window::Theme;
pub type WindowLevel = winit::window::WindowLevel;

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

/// Represents a color
///
/// Initialization with ARGB allows you to create your own colors
#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
    /// Initialization with ARGB allows you to create your own colors
    ARGB(u8, u8, u8, u8),
}

impl Color {
    pub fn inversion(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::ARGB(a, r, g, b) => {
                Color::ARGB(*a, u8::MAX - (*r), u8::MAX - (*g), u8::MAX - (*b))
            }
        }
    }
}

impl Into<acure::Color> for Color {
    fn into(self) -> acure::Color {
        match self {
            Color::Black => acure::Color::ARGB(255, 0, 0, 0),
            Color::White => acure::Color::ARGB(255, 255, 255, 255),
            Color::ARGB(a, r, g, b) => acure::Color::ARGB(a, r, g, b),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ColorPair {
    pub color: Color,
    pub bgr: Color,
    pub shadow: Color,
}

impl ColorPair {
    pub fn new(color: Color, bgr: Color, shadow: Color) -> Self {
        Self { color, bgr, shadow }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub hover: ColorPair,
    pub click: ColorPair,
    pub normal: ColorPair,
    window: WindowTheme,
    pub bgr: Color,
}

impl Theme {
    pub const LIGHT: Theme = Theme {
        hover: ColorPair {
            color: Color::Black,
            bgr: Color::White,
            shadow: Color::ARGB(255, 0, 170, 204),
        },
        click: ColorPair {
            color: Color::Black,
            bgr: Color::ARGB(255, 200, 200, 200),
            shadow: Color::ARGB(255, 0, 70, 204),
        },
        normal: ColorPair {
            color: Color::Black,
            bgr: Color::White,
            shadow: Color::ARGB(255, 128, 128, 128),
        },
        window: WindowTheme::Light,
        bgr: Color::ARGB(255, 240, 240, 240),
    };
    pub const DARK: Theme = Theme {
        hover: ColorPair {
            color: Color::White,
            bgr: Color::ARGB(255, 180, 180, 180),
            shadow: Color::ARGB(255, 200, 200, 200),
        },
        click: ColorPair {
            color: Color::White,
            bgr: Color::ARGB(255, 144, 144, 144),
            shadow: Color::White,
        },
        normal: ColorPair {
            color: Color::ARGB(255, 220, 220, 220),
            bgr: Color::ARGB(255, 72, 72, 72),
            shadow: Color::ARGB(255, 200, 200, 200),
        },
        window: WindowTheme::Dark,
        bgr: Color::ARGB(255, 72, 72, 72),
    };

    pub const LIGHT_HIGH_CONTRAST: Theme = Theme {
        hover: ColorPair {
            color: Color::Black,
            bgr: Color::White,
            shadow: Color::ARGB(255, 0, 108, 255),
        },
        click: ColorPair {
            color: Color::Black,
            bgr: Color::White,
            shadow: Color::ARGB(255, 39, 135, 255),
        },
        normal: ColorPair {
            color: Color::Black,
            bgr: Color::White,
            shadow: Color::ARGB(255, 40, 40, 40),
        },
        window: WindowTheme::Light,
        bgr: Color::White,
    };

    pub const DARK_HIGH_CONTRAST: Theme = Theme {
        hover: ColorPair {
            color: Color::White,
            bgr: Color::Black,
            shadow: Color::ARGB(255, 255, 159, 59),
        },
        click: ColorPair {
            color: Color::White,
            bgr: Color::Black,
            shadow: Color::ARGB(255, 255, 135, 0),
        },
        normal: ColorPair {
            color: Color::White,
            bgr: Color::Black,
            shadow: Color::ARGB(255, 0, 255, 224),
        },
        window: WindowTheme::Dark,
        bgr: Color::Black,
    };
}

impl Theme {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::LIGHT
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WidgetEvent {
    OnHover,
    OnClick,
}

#[derive(Clone, Copy, Debug)]
pub enum ApplicationEvent {
    Close,
    RedrawRequested,
    OnEvent(WidgetEvent, ID),
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

pub struct Application<W>
where
    W: Widget + 'static,
{
    window: Window,
    widget: W,
    theme: Theme,
    render_manager: RenderManager,
}

impl<W> Application<W>
where
    W: Widget + 'static,
{
    pub fn new(window: Window, widget: W) -> Self {
        let render_manager = RenderManager::new(&window.inner);
        Self {
            window,
            widget,
            theme: Theme::LIGHT,
            render_manager,
        }
    }

    pub fn run<F>(mut self, mut callback: F)
    where
        F: FnMut(ApplicationEvent, &mut WidgetRegistry),
    {
        let id = self.widget.id();
        self.widget.theme(self.theme);
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
                                        widget.on_hover();
                                        events.push((*i, WidgetEvent::OnHover))
                                        // self.window.inner.set_cursor_icon(comp.cursor());
                                        // request_redraw = true;
                                        // if get_key_state(VK_LBUTTON) {
                                        //     comp.message(widget::WidgetMessage::OnClick, data);
                                        //     request_redraw = true;
                                        //     std::thread::sleep(Duration::from_millis(200));
                                        // }
                                    } else {
                                        widget.unfocus();
                                    }
                                } else {
                                    widget.unfocus();
                                }
                            }
                        }

                        let mut registry = self.render_manager.registry.lock().unwrap();

                        for (id, mes) in events {
                            callback(ApplicationEvent::OnEvent(mes, id), &mut registry);
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
