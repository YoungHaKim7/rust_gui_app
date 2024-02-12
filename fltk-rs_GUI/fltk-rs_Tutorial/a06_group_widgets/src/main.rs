use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::*,
    prelude::{GroupExt, WidgetExt},
    window::Window,
};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    wind.end();
    wind.show();

    let mut btn = Button::new(160, 210, 80, 40, "Click me!");
    let mut btn2 = Button::new(160, 130, 80, 40, "Click me!");
    wind.add(&btn2);
    wind.add(&btn);

    app.run().unwrap();
}
