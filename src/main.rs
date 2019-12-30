use std::{convert::TryInto, env::args};

use gio::prelude::*;
use gtk::{
    prelude::*, Application, ApplicationWindow, Builder, Button, SpinButton, TextBuffer,
    TextTagTable, TextView,
};

pub mod cereal_simulation {
    use std::time::{Duration, Instant};

    use rand::{thread_rng, Rng};

    pub fn simulation(
        number_of_loops: u128,
    ) -> (Option<usize>, Option<usize>, Option<f64>, Option<Duration>) {
        if number_of_loops == 0 {
            (None, None, None, None)
        } else {
            let mut probability: Vec<usize> = Vec::new();
            // Start timer
            let start = Instant::now();
            // Simulation loop
            for _i in 0..number_of_loops {
                let mut prizes: [usize; 6] = [0, 0, 0, 0, 0, 0];
                let mut opens: usize = 0;
                // loop until owns every prize
                while prizes.contains(&0) {
                    prizes[thread_rng().gen_range(0, 6)] += 1;
                    opens += 1;
                }
                probability.push(opens);
            }
            let max_tries = Some(*probability.iter().max().unwrap());
            let min_tries = Some(*probability.iter().min().unwrap());
            let mean_tries =
                Some(probability.iter().sum::<usize>() as f64 / probability.len() as f64);
            // End timer
            let timer = Some(start.elapsed());
            (max_tries, min_tries, mean_tries, timer)
        }
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
    let spin: SpinButton = builder.get_object("input").expect("Could not get spin.");
    let button: Button = builder.get_object("submit").expect("Could not get submit.");
    button.connect_clicked(move |_| {
        let number_of_loops = spin.get_value_as_int().try_into().unwrap();
        let (max_tries, min_tries, mean_tries, timer) =
            cereal_simulation::simulation(number_of_loops);
        println!(
            "{:?} {:?} {:?} {:?}",
            max_tries.unwrap_or_default(),
            min_tries.unwrap_or_default(),
            mean_tries.unwrap_or_default(),
            timer.unwrap_or_default(),
        );
        let buffer = gtk::TextBuffer::new(gtk::TextTagTable::new());
        //buffer.se
        output.set_buffer(Some(&buffer));
        output.show();
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
