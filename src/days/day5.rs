use crate::common::day::Day;
use crate::common::util::{self};
use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::ops::RangeInclusive;

pub struct Day5;

impl Day for Day5 {
    fn get_name(&self) -> &str {
        "Day 5"
    }

    fn get_input_name(&self) -> &str {
        "day5.txt"
    }

    fn get_description(&self) -> &str {
        "Looks like meat is back on the menu boys"
    }

    //862
    fn solve_part1(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let Some((range, ids)) = input.split_once("\n\n") else {
            return Err("Expected correct input format".into());
        };

        let ranges: Vec<RangeInclusive<_>> =
            util::Util::extract_all_ranges_from_str_range(range, "\n").collect();
        let fresh_count = ids
            .lines()
            .filter(|l| {
                let nr = l.parse::<usize>().expect("Expected a number");
                ranges.iter().any(|r| r.contains(&nr))
            })
            .count();
        Ok(fresh_count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let Some((range, _)) = input.split_once("\n\n") else {
            return Err("Expected correct input format".into());
        };
        let set: HashSet<usize> =
            util::Util::extract_all_numbers_from_str_range(range, "\n").collect();
        Ok(set.iter().count().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::day::Day, days::day5::Day5};

    const EXAMPLE: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    #[test]
    fn part1_example() {
        let day = Day5;
        assert_eq!(day.solve_part1(EXAMPLE).unwrap(), "3")
    }
    #[test]
    fn part2_example() {
        let day = Day5;
        assert_eq!(day.solve_part2(EXAMPLE).unwrap(), "14")
    }
}
