use aom::{Object, ID};

use crate::Application;

use super::{Container, Drawable, Layout, Widget};

pub struct PanelBuilder<W> 
where
    W: Widget + 'static
{
    inner: Vec<Box<W>>
}

impl<W> PanelBuilder<W> 
where
    W: Widget + 'static
{
    pub fn build(self,mut application: Application<W>) -> Panel {
        let registry = application.registry();
        let mut w_id = vec![];
        self.inner.iter().map(|i| {
            w_id.push(i.id());
        });
        for i in self.inner {
            registry.register(i);
        }
        Panel {
            id: ID::from("panel"),
            w_id
        }
    }
}

#[derive(Debug)]
pub struct Panel {
    id: ID,
    w_id: Vec<ID>
}

impl Container for Panel {
    fn format(&mut self) -> Vec<aom::ID> {
        vec![]
    }
}

impl Drawable for Panel {

}

impl Layout for Panel {
    fn area(&self) -> Vec<crate::Rect> {
        vec![]
    }
}

impl Object for Panel {
    fn id(&self) -> aom::ID {
        self.id
    }
}