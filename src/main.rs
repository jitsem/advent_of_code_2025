use crate::common::day::*;
use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::spinner::Spinner;
use clap::Parser;
use clap::ValueEnum;
use day_runner::{DayRun, DayRunner};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

mod common;
mod day_runner;
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
        days[1] = Some(Box::new(Day2));
        days[2] = Some(Box::new(Day3));
        DayFactory { days }
    }
    fn get_day_instance(&self, day_arg: DayArg) -> &Option<Box<dyn Day>> {
        &self.days[(day_arg as isize) as usize]
    }
    fn get_all_day_instances(&self) -> &[Option<Box<dyn Day>>] {
        self.days.as_slice()
    }
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
                Ok(input) => Ok(DayRun::new(*di, input)),
                Err(e) => Err(e),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let DayRunner = DayRunner;
    let spinner = Spinner::new();
    let run_result = DayRunner::run_set(&days_to_run, &spinner);
    spinner.stop_spinner();
    match run_result {
        Ok(_) => {
            println!("Advented succesfully!");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed during advent!");
            Err(e)
        }
    }
}
