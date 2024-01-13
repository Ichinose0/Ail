mod button;

use std::fmt::Debug;

pub use button::*;

use aom::Object;
use acure::Command;

pub trait EventListener<M> 
where
    M: Clone + Copy + Debug
{
    fn on_click(&mut self) -> Option<M> { None }
    fn on_hover(&mut self) -> Option<M> { None }
    fn on_update(&mut self) -> Option<M> { None }
}

pub trait Drawable {
    fn render(&mut self) -> Vec<Command> {
        vec![]
    }
}

pub trait Widget<M>: std::fmt::Debug + Object + Drawable + EventListener<M> 
where
    M: Clone + Copy + Debug
{

}