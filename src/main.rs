use clap::Parser;
use clap::ValueEnum;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use crate::common::day::*;
use crate::days::day1::Day1;
use crate::spinner::Spinner;

mod common;
mod days;
mod spinner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The day to run, if not present all days are ran
    #[arg(value_enum)]
    day: Option<DayArg>,

    /// Sets a custom input folder location
    #[arg(short, long, value_name = "FILE")]
    input_folder: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[repr(usize)]
enum DayArg {
    Day1 = 0,
    Day2 = 1,
    Day3 = 2,
    Day4 = 3,
    Day5 = 4,
    Day6 = 5,
    Day7 = 6,
    Day8 = 7,
    Day9 = 8,
    Day10 = 9,
    Day11 = 10,
    Day12 = 11,
}

struct DayFactory {
    days: [Option<Box<dyn Day>>; 12],
}
impl DayFactory {
    fn new() -> Self {
        let mut days: [Option<Box<dyn Day>>; 12] = [const { None }; 12];
        days[0] = Some(Box::new(Day1));
        DayFactory { days }
    }
    fn get_day_instance(&self, day_arg: DayArg) -> &Option<Box<dyn Day>> {
        &self.days[(day_arg as isize) as usize]
    }
    fn get_all_day_instances(&self) -> &[Option<Box<dyn Day>>] {
        self.days.as_slice()
    }
}

struct DayRun<'a> {
    instance: &'a dyn Day,
    input: String,
}

struct DayResult {
    name: String,
    description: String,
    part_1: String,
    part_2: String,
    duration: Duration,
}

fn load_input(path: &Path, file_name: &str) -> Result<String, Box<dyn Error>> {
    let mut buff = path.to_path_buf();
    buff.push(file_name);
    let content = fs::read_to_string(buff)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let root_input_path = args.input_folder.unwrap_or(PathBuf::from("input"));
    let instance_fac = DayFactory::new();
    let days_instances = match args.day {
        Some(day) => {
            let day = instance_fac.get_day_instance(day);
            vec![&**(day.as_ref().expect("Day {day} is not yet implemented"))]
        }
        None => instance_fac
            .get_all_day_instances()
            .iter()
            .filter_map(|op| op.as_ref())
            .map(|b| &**b)
            .collect::<Vec<&dyn Day>>(),
    };
    let days_to_run = days_instances
        .iter()
        .map(|di| {
            let input = load_input(&root_input_path, di.get_input_name());
            match input {
                Ok(input) => Ok(DayRun {
                    input,
                    instance: *di,
                }),
                Err(e) => Err(e),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut results: Vec<DayResult> = Vec::with_capacity(days_to_run.len());
    let spinner = Spinner::new();
    let run_result = run_set(&days_to_run, &mut results, &spinner);
    spinner.stop_spinner();
    match run_result {
        Ok(_) => {
            println!("Advented succesfully!");
            pretty_print_slice_of_day_result(&results);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed during advent!");
            Err(e)
        }
    }
}

fn run_set(
    days_to_run: &[DayRun],
    results: &mut Vec<DayResult>,
    spinner: &Spinner,
) -> Result<(), Box<dyn Error>> {
    for day_to_run in days_to_run.iter() {
        let name = day_to_run.instance.get_name();
        let description = day_to_run.instance.get_description();
        pretty_print_name_description(name, description);
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
        pretty_print_day_result(&res);
        results.push(res);
    }
    Ok(())
}

fn pretty_print_slice_of_day_result(results: &[DayResult]) {
    println!("Executed following days:");
    for result in results {
        println!("\t{} - {}", result.name, result.description)
    }
    let total_time: u128 = results.iter().map(|r| r.duration.as_millis()).sum();
    println!("Total took {total_time}ms!")
}

fn pretty_print_name_description(name: &str, description: &str) {
    println!("-----------------------------------------");
    println!("{} - {}", name, description);
    println!("-----------------------------------------");
}

fn pretty_print_day_result(result: &DayResult) {
    println!("Part 1: {}", result.part_1);
    println!("Part 2: {}", result.part_2);
    println!("Took: {}ms", result.duration.as_millis());
    println!("=========================================");
}
