use std::fmt::Debug;

use acure::{Color, Command};
use aom::{Object, ID};

use crate::{Rect, Theme};

use super::{Drawable, EventListener, Layout, Widget, WidgetState};

#[derive(Debug)]
pub struct Button {
    id: ID,
    text: String,
    theme: Theme,
    state: WidgetState,
}

impl Button {
    pub fn new(id: &'static str) -> Self {
        Self {
            id: ID::from(id),
            text: String::from("Button"),
            theme: Theme::LIGHT,
            state: WidgetState::Unfocus,
        }
    }

    pub fn set_text<T>(&mut self, text: T)
    where
        T: Into<String>,
    {
        self.text = text.into();
    }
}

impl Widget for Button {}

impl Layout for Button {
    fn area(&self) -> Vec<Rect> {
        vec![Rect::from_coordinate(10, 10, 240, 40)]
    }
}

impl Drawable for Button {
    fn theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    fn render(&mut self) -> Vec<acure::Command> {
        let (bgr, color, shadow) = match self.state {
            WidgetState::Hover => (
                self.theme.hover.bgr,
                self.theme.hover.color,
                self.theme.hover.shadow,
            ),
            WidgetState::Click => (
                self.theme.click.bgr,
                self.theme.click.color,
                self.theme.click.shadow,
            ),
            WidgetState::Unfocus => (
                self.theme.normal.bgr,
                self.theme.normal.color,
                self.theme.normal.shadow,
            ),
        };

        vec![
            Command::FillRectangle(10, 10, 240, 40, 4.2, shadow.into()),
            Command::FillRectangle(10 + 1, 10 + 1, 240 - 2, 40 - 2, 4.2, bgr.into()),
            Command::WriteString(10, 10, 240, 40, color.into(), self.text.clone()),
        ]
    }
}

impl EventListener for Button {
    fn on_click(&mut self) {}
    fn on_hover(&mut self) {
        self.state = WidgetState::Hover;
    }
    fn unfocus(&mut self) {
        self.state = WidgetState::Unfocus;
    }
    fn on_update(&mut self) {}
}

impl Object for Button {
    fn id(&self) -> ID {
        self.id
    }
}
