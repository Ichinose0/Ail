use std::fmt::Debug;

use aom::ID;

use crate::{ApplicationEvent, widget::Widget};

#[derive(Clone,Copy,Debug)]
pub enum EventKind<M> 
where
    M: Clone + Copy + Debug
{
    Redraw,
    User(M),
    Application(ApplicationEvent<M>)
}

pub struct EventRequest<M>
where
    M: Clone + Copy + Debug
{
    id: ID,
    kind: EventKind<M>
}

impl<M> EventRequest<M> 
where
    M: Clone + Copy + Debug
{
    pub fn new(widget: &impl Widget<M>,kind: EventKind<M>) -> Self {
        Self {
            id: widget.id(),
            kind
        }
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn kind(&self) -> EventKind<M> {
        self.kind
    }
}