use fltk::{
    app, button, frame, group, input, menu,
    prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt},
    window::Window,
};

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut choice = menu::Choice::default()
        .with_size(80, 30)
        .center_of_parent()
        .with_label("Select item");
    choice.add_choice("Choice 1");
    choice.add_choice("Choice 2");
    choice.add_choice("Choice 3");

    wind.end();
    wind.show();

    choice.set_callback(|c| match c.value() {
        0 => println!("choice 1 selected"),
        1 => println!("choice 2 selected"),
        2 => println!("choice 3 selected"),
        _ => unreachable!(),
    });

    app.run().unwrap();
}
