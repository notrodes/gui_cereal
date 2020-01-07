use gio::prelude::*;
use gtk::{prelude::*, Application, ApplicationWindow, Builder, Button, SpinButton, TextView};
use gui_cereal::cereal_simulation;
use std::env::args;

fn build_app(parent_app: &Application) {
    const UI: &str = include_str!("builder_basics.ui");
    let builder = Builder::new_from_string(UI);
    let window: ApplicationWindow = builder.get_object("window").expect("Could not get window.");
    window.set_application(Some(parent_app));
    window.show_all();
    let output: TextView = builder.get_object("output").expect("Could not get output.");
    output.hide();
    let spin: SpinButton = builder.get_object("input").expect("Could not get spin.");
    let button: Button = builder.get_object("submit").expect("Could not get submit.");
    button.connect_clicked(move |_| {
        let number_of_loops = spin.get_value_as_int();
        let (max_tries, min_tries, mean_tries, timer) =
            cereal_simulation::simulation(number_of_loops, true);
        println!(
            "{:?} {:?} {:?} {:?}",
            max_tries.unwrap_or_default(),
            min_tries.unwrap_or_default(),
            mean_tries.unwrap_or_default(),
            timer.unwrap_or_default(),
        );
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| build_app(app));
    application.run(&args().collect::<Vec<_>>());
}
