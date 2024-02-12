use fltk::{
    app, button, frame, group, input,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut pack = group::Pack::default_fill();
    pack.set_spacing(5);
    for i in 0..2 {
        frame::Frame::default()
            .with_size(0, 40)
            .with_label(&format!("filed {}", i));
        input::Input::default().with_size(0, 40);
    }
    frame::Frame::default().with_size(0, 40); // a filler
    button::Button::default()
        .with_size(0, 40)
        .with_label("Submit!");
    pack.end();
    wind.end();
    wind.show();

    app.run().unwrap();
}
