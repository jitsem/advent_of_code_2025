use crate::common::day::Day;
use crate::spinner::Spinner;
use std::error::Error;
use std::time::{Duration, Instant};

pub struct DayRun<'a> {
    instance: &'a dyn Day,
    input: String,
}

impl<'a> DayRun<'a> {
    pub fn new(instance: &'a dyn Day, input: String) -> Self {
        Self { instance, input }
    }
}

pub struct DayResult {
    name: String,
    description: String,
    part_1: String,
    part_2: String,
    duration: Duration,
}

pub struct DayRunner;

impl DayRunner {
    pub fn run_set(days_to_run: &[DayRun], spinner: &Spinner) -> Result<(), Box<dyn Error>> {
        let mut results: Vec<DayResult> = Vec::with_capacity(days_to_run.len());
        for day_to_run in days_to_run.iter() {
            let name = day_to_run.instance.get_name();
            let description = day_to_run.instance.get_description();
            Self::pretty_print_name_description(name, description);
            spinner.resume_spining();
            let now = Instant::now();
            let part_1 = day_to_run.instance.solve_part1(&day_to_run.input)?;
            let part_2 = day_to_run.instance.solve_part2(&day_to_run.input)?;
            let duration = now.elapsed();
            let res = DayResult {
                name: name.into(),
                description: description.into(),
                part_1,
                part_2,
                duration,
            };
            spinner.pause_spining();
            Self::pretty_print_day_result(&res);
            results.push(res);
        }
        Self::pretty_print_slice_of_day_result(results.as_slice());
        Ok(())
    }

    fn pretty_print_slice_of_day_result(results: &[DayResult]) {
        println!("Executed following days:");
        for result in results {
            println!("\t{} - {}", result.name, result.description)
        }
        let total_time: u128 = results.iter().map(|r| r.duration.as_micros()).sum();
        println!("Total took {:.3}ms!", total_time as f64 / 1000f64)
    }

    fn pretty_print_name_description(name: &str, description: &str) {
        println!("-----------------------------------------");
        println!("{} - {}", name, description);
        println!("-----------------------------------------");
    }

    fn pretty_print_day_result(result: &DayResult) {
        println!("Part 1: {}", result.part_1);
        println!("Part 2: {}", result.part_2);
        println!("Took: {:.3}ms", result.duration.as_micros() as f64/ 1000f64);
        println!("=========================================");
    }
}
