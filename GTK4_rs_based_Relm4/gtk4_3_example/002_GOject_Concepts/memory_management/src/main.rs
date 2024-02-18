use std::{cell::Cell, rc::Rc};

use gtk::{
    glib::clone,
    prelude::*,
    Button, {glib, Application, ApplicationWindow},
};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(30)
        .margin_bottom(30)
        .margin_start(12)
        .margin_end(12)
        .build();

    let mut number = Rc::new(Cell::new(0));

    let number_copy = number.clone();
    button_increase.connect_clicked(clone!(@weak number, @strong button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@strong button_increase =>
        move |_| {
        number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}
