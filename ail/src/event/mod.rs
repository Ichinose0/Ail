use aom::ID;

use crate::{ApplicationEvent, widget::Widget};

#[derive(Clone,Copy,Debug)]
pub enum EventKind {
    Redraw,
    Application(ApplicationEvent)
}

pub struct EventRequest {
    id: ID,
    kind: EventKind
}

impl EventRequest {
    pub fn new(widget: &impl Widget,kind: EventKind) -> Self {
        Self {
            id: widget.id(),
            kind
        }
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn kind(&self) -> EventKind {
        self.kind
    }
}