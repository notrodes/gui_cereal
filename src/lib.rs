pub mod cereal_simulation {
    use rand::{thread_rng, Rng};
    use std::{
        sync::mpsc::Sender,
        time::{Duration, Instant},
    };

    fn calculate(number_of_loops: u128, tx: Sender<i32>) -> Vec<usize> {
        let mut probability: Vec<usize> = Vec::new();
        // Simulation loop
        for _i in 0..number_of_loops {
            let mut prizes: [usize; 6] = [0, 0, 0, 0, 0, 0];
            let mut opens: usize = 0;
            // loop until owns every prize
            while prizes.contains(&0) {
                prizes[thread_rng().gen_range(0, 6)] += 1;
                opens += 1;
                // Send signal one loop has gone
                tx.send(1).unwrap();
            }
            probability.push(opens);
        }
        probability
    }

    pub fn simulation(
        number_of_loops: u128,
        tx: Sender<i32>,
    ) -> (Option<usize>, Option<usize>, Option<f64>, Option<Duration>) {
        if number_of_loops == 0 {
            (None, None, None, None)
        } else {
            // Start timer
            let start = Instant::now();
            let probability = calculate(number_of_loops, tx);
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
