use crate::common::day::Day;
use crate::common::util;
use std::error::Error;

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
        let doubles: u64 = util::Util::extract_all_numbers_from_str_range(input, ",")
            .map(|i| i.to_string())
            .filter_map(|n| {
                if n.len() % 2 == 0 {
                    let div = n.len() / 2;
                    let (one, two) = n.split_at(div);
                    if one == two {
                        return n.parse::<u64>().ok();
                    }
                }
                None
            })
            .sum();
        Ok(doubles.to_string())
    }

    //22471660255
    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let doubles: u64 = util::Util::extract_all_numbers_from_str_range(input, ",")
            .map(|i| i.to_string())
            .filter_map(|n| {
                let same = (1..=n.len() / 2).any(|r| {
                    let (left, right) = n.split_at(r);
                    right
                        .as_bytes()
                        .chunks(left.len())
                        .all(|chunk| chunk == left.as_bytes())
                });

                if same { n.parse::<u64>().ok() } else { None }
            })
            .sum();
        Ok(doubles.to_string())
    }
}

impl Day2 {}

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
