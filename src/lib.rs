pub mod cereal_simulation {
    use rand::{thread_rng, Rng};
    use std::{
        thread,
        time::{Duration, Instant},
    };

    fn calculate(number_of_loops: i32) -> Option<Vec<i32>> {
        if number_of_loops == 0 {
            return None;
        }
        let mut probability: Vec<i32> = Vec::new();
        // Simulation loop
        for _i in 0..number_of_loops {
            let mut prizes: [i32; 6] = [0, 0, 0, 0, 0, 0];
            let mut opens: i32 = 0;
            // loop until owns every prize
            while prizes.contains(&0) {
                prizes[thread_rng().gen_range(0, 6)] += 1;
                opens += 1;
            }
            probability.push(opens);
        }
        Some(probability)
    }

    pub fn simulation(
        number_of_loops: i32,
        concurrent: bool,
    ) -> (Option<i32>, Option<i32>, Option<f64>, Option<Duration>) {
        if number_of_loops == 0 {
            (None, None, None, None)
        } else {
            let start;
            let probability;
            if concurrent {
                // Start timer
                start = Instant::now();
                let child_number_of_loops = number_of_loops.clone();
                let child_load = (child_number_of_loops as f32 / 2 as f32).floor() as i32;
                let child = thread::spawn(move || calculate(child_load));
                let load = (number_of_loops as f32 / 2 as f32).floor() as i32;
                probability = calculate(load);

                let mut child = child.join().unwrap().unwrap();
                probability.unwrap().append(&mut child);
            } else {
                start = Instant::now();
                probability = calculate(number_of_loops);
            }
            // End timer
            let timer = Some(start.elapsed());
            let probability = probability.clone();
            let probability = probability.unwrap();
            let max_tries = Some(*probability.iter().max().unwrap());
            let min_tries = Some(*probability.iter().min().unwrap());
            let mean_tries =
                Some(probability.iter().sum::<i32>() as f64 / probability.len() as f64);
            (max_tries, min_tries, mean_tries, timer)
        }
    }
}
