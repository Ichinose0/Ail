use std::fmt::Debug;

use acure::{Command, Color};
use aom::{ID,Object};

use super::{Widget, Drawable, EventListener};

#[derive(Debug)]
pub struct Button<M> 
where
    M: Clone + Copy + Debug
{
    id: ID,
    text: String,
    on_click: Option<M>
}

impl<M> Button<M> 
where
    M: Clone + Copy + Debug
{
    pub fn new(id: &'static str) -> Self {
        Self {
            id: ID::from(id),
            text: String::from("Button"),
            on_click: None
        }
    }

    pub fn set_text<T>(&mut self,text: T) 
    where
        T: Into<String>
    {
        self.text = text.into();
    }
}

impl<M> Widget<M> for Button<M>
where
    M: Clone + Copy + Debug
{
    
}

impl<M> Drawable for Button<M> 
where
    M: Clone + Copy + Debug
{
    fn render(&mut self) -> Vec<acure::Command> {
        vec![
            Command::FillRectangle(
                10,
                10,
                240,
                40,
                4.2,
                Color::ARGB(255,128,128,128),
            ),
            Command::FillRectangle(
                10 + 1,
                10 + 1,
                240 - 2,
                40 - 2,
                4.2,
                Color::ARGB(255,240,240,240),
            ),
            Command::WriteString(
                10,
                10,
                240,
                40,
                Color::ARGB(255,0,0,0),
                self.text.clone(),
            ),
        ]
    }
}

impl<M> EventListener<M> for Button<M> 
where
    M: Clone + Copy + Debug
{
    fn on_click(&mut self) -> Option<M> { None }
    fn on_hover(&mut self) -> Option<M> { None }
    fn on_update(&mut self) -> Option<M> { None }
}

impl<M> Object for Button<M> 
where
    M: Clone + Copy + Debug
{
    fn id(&self) -> ID {
        self.id
    }
}