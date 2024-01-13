mod button;

use std::fmt::Debug;

pub use button::*;

use acure::Command;
use aom::Object;

use crate::{Rect, Theme};

#[derive(Clone, Copy, Debug)]
pub enum WidgetState {
    Hover,
    Click,
    Unfocus,
}

pub trait EventListener {
    fn on_click(&mut self) {}
    fn on_hover(&mut self) {}
    fn unfocus(&mut self) {}
    fn on_update(&mut self) {}
}

pub trait Drawable {
    fn theme(&mut self, theme: Theme) {}

    fn render(&mut self) -> Vec<Command> {
        vec![]
    }
}

pub trait Layout {
    fn area(&self) -> Vec<Rect>;
}

pub trait Widget: std::fmt::Debug + Object + Drawable + Layout + EventListener {}
