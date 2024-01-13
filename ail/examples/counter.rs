use ail::{Window, Application, widget::{Widget, Button}};
use aom::Object;

#[derive(Clone,Copy,Debug)]
pub enum Message {

}

fn main() {
    let window = Window::new();
    let mut button:Button<Message> = Button::new("my_button");
    button.set_text(button.id());
    let app = Application::new(window,button);
    app.run(|event | {

    });
}