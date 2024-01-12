use aom::{ID, Object};
use management::WidgetRegistry;
use widget::Button;

pub mod management;
pub mod widget;
pub mod event;

pub type CursorIcon = winit::window::CursorIcon;

#[derive(Clone,Copy,Debug)]
pub enum ApplicationEvent {
    Close,
    SetCursor(CursorIcon)
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
