use crate::common::day::Day;
use std::error::Error;

pub struct Day3;

impl Day for Day3 {
    fn get_name(&self) -> &str {
        "Day 3"
    }

    fn get_input_name(&self) -> &str {
        "day3.txt"
    }

    fn get_description(&self) -> &str {
        "Powering the escalator"
    }

    fn solve_part1(&self, input: &str) -> Result<String, Box<dyn Error>> {
        Ok(Self::get_battey_bank_iter(input)
            .map(|chars| {
                let first_max = chars[..chars.len() - 1]
                    .iter()
                    .enumerate()
                    .rev() //max_by_key select last occuring
                    .max_by_key(|(_, e)| **e)
                    .expect("Iterator is not empty");
                let second_max = chars
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i > first_max.0)
                    .rev() //max_by_key select last occuring
                    .max_by_key(|(_, e)| **e)
                    .expect("Iterator is not empty");
                u64::from(*first_max.1 - 48) * 10 + u64::from(*second_max.1 - 48)
            })
            .fold(0u64, |acc, e| acc + e)
            .to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        Ok("Ok".to_string())
    }
}

impl Day3 {
    fn get_battey_bank_iter(input: &str) -> impl Iterator<Item = &[u8]> {
        input.lines().map(|l| l.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::day::Day, days::day3::Day3};
    #[test]
    fn part1_example() {
        let day = Day3;
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(day.solve_part1(input).unwrap(), "357")
    }

    #[test]
    fn part1_another_example() {
        let day = Day3;
        let input = "25512224224\n";
        assert_eq!(day.solve_part1(input).unwrap(), "55")
    }
    // #[test]
    // fn part2_example() {
    //     let day = Day3;
    //     let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
    //     assert_eq!(day.solve_part2(input).unwrap(), "TODO")
    // }
}
