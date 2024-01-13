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
    button.set_text(button.id());
    button.on_hover(Message::ButtonHovered);
    let app = Application::new(window, button);
    app.run(|event| {
        match event {
            ApplicationEvent::Message(mes) => {
                println!("{:?}",mes);
            }
            _ => {}
        }
    });
}
