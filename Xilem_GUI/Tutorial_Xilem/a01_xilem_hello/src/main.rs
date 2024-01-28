use xilem::{
    view::{button, h_stack, switch, v_stack, View},
    App, AppLauncher,
};

struct AppData {
    count: i32,
    is_on: bool,
}

fn app_logic(data: &mut AppData) -> impl View<AppData> {
    let count = data.count;
    let label = if count == 1 {
        "clicked 1 time".to_string()
    } else {
        format!("clicked {count} tiems")
    };

    v_stack((
        button(label, |data: &mut AppData| {
            println!("clicked");
            data.count += 1;
        }),
        h_stack((
            button("decrease", |data: &mut AppData| {
                println!("clicked decrease");
                data.count -= 1;
            }),
            button("reset", |data: &mut AppData| {
                println!("clicked reset");
                data.count = 0;
            }),
            switch(data.is_on, |data: &mut AppData, value: bool| {
                data.is_on = value
            }),
        )),
    ))
    .with_spacing(20.0)
}

fn main() {
    let data = AppData {
        count: 0,
        is_on: false,
    };

    let app = App::new(data, app_logic);
    AppLauncher::new(app).run()
}
