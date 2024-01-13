mod button;

pub use button::*;

use aom::Object;
use acure::Command;

pub trait Drawable {
    fn render(&mut self) -> Vec<Command> {
        vec![]
    }
}

pub trait Widget: std::fmt::Debug + Object + Drawable {

}