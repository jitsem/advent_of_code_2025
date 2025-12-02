use std::error::Error;
use clap::builder::TypedValueParser;
use fancy_regex::Regex;
use crate::common::day::Day;

pub struct Day2;

impl Day for Day2 {
    fn get_name(&self) -> &str {
        "Day 2"
    }

    fn get_input_name(&self) -> &str {
        "day2.txt"
    }

    fn get_description(&self) -> &str {
        "The souvenir shop"
    }

    //13108371860
    fn solve_part1(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let numbers = Self::extract_all_numbers(input)?;
        let re = Regex::new(r"^(\d.*)\1$").unwrap();
        let res = Self::count_matches(numbers, re)?;
        Ok(res.to_string())
    }

    //22471660255
    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let numbers = Self::extract_all_numbers(input)?;
        let re = Regex::new(r"^(\d.*)\1+$").unwrap();
        let res = Self::count_matches(numbers, re)?;
        Ok(res.to_string())
    }
}

impl Day2 {
    fn extract_all_numbers(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let ranges = input.split(",").into_iter().collect::<Vec<_>>();
        let mut numbers: Vec<String> = Vec::new();
        for range in ranges {
            let range = range.split_once("-").expect("Expected x - y");
            let lower = range.0.parse::<usize>()?;
            let upper = range.1.parse::<usize>()?;
            for i in lower..upper + 1 {
                numbers.push(i.to_string())
            }
        }
        Ok(numbers)
    }

    fn count_matches(numbers: Vec<String>, re: Regex) -> Result<u64, Box<dyn Error>> {
        let mut res = 0;
        for n in numbers {
            let result = re.is_match(n.as_str())?;
            if result {
                let number: u64 = n.parse()?;
                res += number
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::day::Day, days::day2::Day2};
    #[test]
    fn part1_example() {
        let day = Day2;
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(day.solve_part1(input).unwrap(), "1227775554")
    }
    #[test]
    fn part2_example() {
        let day = Day2;
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(day.solve_part2(input).unwrap(), "4174379265")
    }
}