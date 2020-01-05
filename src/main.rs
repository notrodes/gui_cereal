use futures::executor::block_on;
use gio::prelude::*;
use gtk::{
    prelude::*, Application, ApplicationWindow, Builder, Button, ProgressBar, SpinButton, TextView,
};
use gui_cereal::cereal_simulation;
use std::{
    env::args,
    sync::mpsc::{self, Receiver},
};

async fn increment_progress_bar(
    progress_bar: &ProgressBar,
    number_of_loops: u128,
    rx: Receiver<i32>,
) {
    let mut count = 0;
    progress_bar.set_fraction(0.0);
    while count < number_of_loops {
        println!("{}", rx.recv().unwrap());
        count += 1;
    }
}

fn build_app(parent_app: &Application) {
    const UI: &str = include_str!("builder_basics.ui");
    let builder = Builder::new_from_string(UI);
    let window: ApplicationWindow = builder.get_object("window").expect("Could not get window.");
    window.set_application(Some(parent_app));
    window.show_all();
    let output: TextView = builder.get_object("output").expect("Could not get output.");
    output.hide();
    let progress: ProgressBar = builder
        .get_object("progress")
        .expect("Could not get progress.");
    progress.hide();
    let spin: SpinButton = builder.get_object("input").expect("Could not get spin.");
    let button: Button = builder.get_object("submit").expect("Could not get submit.");
    button.connect_clicked(move |_| {
        let (tx, rx) = mpsc::channel();
        let number_of_loops: u128 = spin.get_value_as_int() as u128;
        let (max_tries, min_tries, mean_tries, timer) =
            cereal_simulation::simulation(number_of_loops, tx);
        println!(
            "{:?} {:?} {:?} {:?}",
            max_tries.unwrap_or_default(),
            min_tries.unwrap_or_default(),
            mean_tries.unwrap_or_default(),
            timer.unwrap_or_default(),
        );
        block_on(increment_progress_bar(&progress, number_of_loops, rx))
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
