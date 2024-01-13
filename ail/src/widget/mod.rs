mod button;

use std::fmt::Debug;

pub use button::*;

use acure::Command;
use aom::Object;

use crate::{Rect, Theme};

#[derive(Clone,Copy,Debug)]
pub enum WidgetState {
    Hover,
    Click,
    Unfocus
}

pub trait EventListener<M>
where
    M: Clone + Copy + Debug,
{
    fn on_click(&mut self) -> Option<M> {
        None
    }
    fn on_hover(&mut self) -> Option<M> {
        None
    }
    fn unfocus(&mut self) {
        
    }
    fn on_update(&mut self) -> Option<M> {
        None
    }
}

pub trait Drawable {
    fn theme(&mut self,theme: Theme) {

    }

    fn render(&mut self) -> Vec<Command> {
        vec![]
    }
}

pub trait Layout {
    fn area(&self) -> Vec<Rect>;
}

pub trait Widget<M>: std::fmt::Debug + Object + Drawable + Layout + EventListener<M>
where
    M: Clone + Copy + Debug,
{
}
