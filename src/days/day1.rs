use crate::common::day::Day;
use std::{error::Error, thread::sleep, time::Duration};

pub struct Day1;
impl Day for Day1 {
    fn solve_part1(&self, _: &str) -> Result<String, Box<dyn Error>> {
        sleep(Duration::from_secs(2));
        Ok("placeholder".to_owned())
    }

    fn solve_part2(&self, _: &str) -> Result<String, Box<dyn Error>> {
        sleep(Duration::from_secs(2));
        Ok("placeholder".to_owned())
    }

    fn get_name(&self) -> &str {
        "Day 1"
    }

    fn get_input_name(&self) -> &str {
        "day1.txt"    }

    fn get_description(&self) -> &str {
        "A testing day"
    }
}
