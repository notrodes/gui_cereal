use gio::{prelude::*, ApplicationFlags};
use gtk::{prelude::*, Application, ApplicationWindow, Button, ButtonBox};
use std::env;

fn main() {
    let app: Application = gtk::Application::new(
        Some("com.github.notrodes.gtk_hello_world"),
        ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    app.connect_activate(|app: &Application| {
        let window: ApplicationWindow = gtk::ApplicationWindow::new(app);
        window.set_default_size(1000, 600);
        window.set_title("Hello World!");
        let button_box: ButtonBox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        window.add(&button_box);
        let button: Button = gtk::Button::new_with_label("Hello World!");
        button_box.add(&button);
        window.show_all();
    });
    app.run(&env::args().collect::<Vec<_>>());
}