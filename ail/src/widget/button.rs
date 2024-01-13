use acure::{Command, Color};
use aom::{ID,Object};

use super::{Widget, Drawable};

#[derive(Debug)]
pub struct Button {
    id: ID,
    text: String
}

impl Button {
    pub fn new(id: &'static str) -> Self {
        Self {
            id: ID::from(id),
            text: String::from("Button")
        }
    }

    pub fn set_text(&mut self,text: String) {
        self.text = text;
    }
}

impl Widget for Button {
    
}

impl Drawable for Button {
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

impl Object for Button {
    fn id(&self) -> ID {
        self.id
    }
}