use ail::{
    widget::{Button, Widget},
    Application, Window, ApplicationEvent,
};
use aom::Object;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonHovered
}

fn main() {
    let window = Window::new();
    let mut button: Button<Message> = Button::new("my_button");
    let id = button.id();
    button.set_text(button.id());
    button.on_hover(Message::ButtonHovered);
    let app = Application::new(window, button);
    app.run(|event,registry| {
        match event {
            ApplicationEvent::Message(mes) => {
                let button:&mut Box<Button<Message>> = registry.get_mut(id).unwrap();
                button.set_text("Clicked");
            }
            _ => {}
        }
    });
}
