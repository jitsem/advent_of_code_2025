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
                Self::find_max_contigonous_in_slice(chars, 2).expect("We assume each line > 2")
            })
            .sum::<u64>()
            .to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        Ok(Self::get_battey_bank_iter(input)
            .map(|chars| {
                Self::find_max_contigonous_in_slice(chars, 12).expect("We assume each line > 12")
            })
            .sum::<u64>()
            .to_string())
    }
}

impl Day3 {
    fn get_battey_bank_iter(input: &str) -> impl Iterator<Item = &[u8]> {
        input.lines().map(|l| l.as_bytes())
    }

    fn find_max_contigonous_in_slice(
        slice: &[u8],
        number_of_digits: usize,
    ) -> Result<u64, Box<dyn Error>> {
        if number_of_digits > slice.len() {
            return Err("Cannot have more digits than slice len".into());
        }
        let mut prev_idx: Option<usize> = None;
        Ok((0..number_of_digits)
            .rev()
            .map(|idx| {
                let max = slice[..(slice.len() - idx)]
                    .iter()
                    .enumerate()
                    .rev() //max_by_key select last occuring
                    .filter(|(i, _)| {
                        if let Some(prev_idx) = prev_idx {
                            *i > prev_idx
                        } else {
                            true
                        }
                    })
                    .max_by_key(|(_, e)| **e)
                    .expect("Iterator is not empty");
                prev_idx = Some(max.0);
                u64::from(max.1 - 48)
            })
            .fold(0u64, |acc, e| (acc * 10) + e))
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
    #[test]
    fn part2_example() {
        let day = Day3;
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(day.solve_part2(input).unwrap(), "3121910778619")
    }
}
