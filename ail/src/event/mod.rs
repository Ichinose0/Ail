use std::fmt::Debug;

use aom::ID;

use crate::{ApplicationEvent, widget::Widget};

#[derive(Clone,Copy,Debug)]
pub enum EventKind<E> 
where
    E: Clone + Copy + Debug
{
    Redraw,
    User(E),
    Application(ApplicationEvent)
}

pub struct EventRequest<E>
where
    E: Clone + Copy + Debug
{
    id: ID,
    kind: EventKind<E>
}

impl<E> EventRequest<E> 
where
    E: Clone + Copy + Debug
{
    pub fn new(widget: &impl Widget,kind: EventKind<E>) -> Self {
        Self {
            id: widget.id(),
            kind
        }
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn kind(&self) -> EventKind<E> {
        self.kind
    }
}