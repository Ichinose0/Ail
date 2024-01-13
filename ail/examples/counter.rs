use ail::{
    widget::{Button, Widget},
    Application, ApplicationEvent, Window,
};
use aom::Object;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    ButtonHovered,
}

fn main() {
    let window = Window::new();
    let mut button = Button::new("my_button");
    let button_id = button.id();
    button.set_text(button.id());
    let app = Application::new(window, button);
    app.run(|event, registry| match event {
        ApplicationEvent::OnEvent(mes, id) => match mes {
            ail::WidgetEvent::OnHover => {
                if id == button_id {
                    let button: &mut Box<Button> = registry.get_mut(id).unwrap();
                    button.set_text("Hovered");
                }
            }
            ail::WidgetEvent::OnClick => {}
        },
        _ => {}
    });
}
