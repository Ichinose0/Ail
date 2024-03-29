use acure::{d2d1::D2D1Surface, surface::Surface, Acure, AlignMode, Color, Command, LayoutMode};
use winit::{raw_window_handle::HasWindowHandle, window::Window};

use crate::widget::Widget;

pub struct Renderer {
    acure: Acure,
    inner: Box<dyn Surface>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let handle = window.window_handle().unwrap();

        let acure = Acure::new();
        let inner = Box::new(match handle.as_raw() {
            winit::raw_window_handle::RawWindowHandle::UiKit(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::AppKit(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Orbital(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Xlib(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Xcb(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Wayland(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Drm(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Gbm(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Win32(handle) => unsafe {
                D2D1Surface::new(isize::from(handle.hwnd)).unwrap()
            },
            winit::raw_window_handle::RawWindowHandle::WinRt(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Web(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::WebCanvas(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::WebOffscreenCanvas(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::AndroidNdk(_) => todo!(),
            winit::raw_window_handle::RawWindowHandle::Haiku(_) => todo!(),
            _ => todo!(),
        });

        Self { acure, inner }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.inner.surface_resize(width, height);
    }

    pub fn begin(&mut self) {
        self.inner.begin();
        self.inner.clear(Color::ARGB(255,240,240,240));
    }

    pub fn end(&mut self) {
        self.inner.end();
    }

    pub fn render<W>(&mut self, mut widget: &mut Box<W>)
    where
        W: Widget + ?Sized,
    {
        for i in &widget.render() {
            self.inner
                .command(i, AlignMode::CenterAligned, LayoutMode::AdjustSize)
        }
    }
}
