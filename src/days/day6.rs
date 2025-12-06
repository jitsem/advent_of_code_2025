use crate::common::day::Day;
use std::error::Error;

pub struct Day6;

impl Day for Day6 {
    fn get_name(&self) -> &str {
        "Day 6"
    }

    fn get_input_name(&self) -> &str {
        "day6.txt"
    }

    fn get_description(&self) -> &str {
        "Snail Math"
    }

    fn solve_part1(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let lines = input.lines();
        let operation_line = lines.last().expect("Expected a last line");
        let operations = operation_line
            .split_ascii_whitespace()
            .map(|str| match str {
                "+" => Operation::Plus,
                "*" => Operation::Multiply,
                _ => panic!("Unexpected operation"),
            });
        let count: usize = operations
            .enumerate()
            .map(|(i, op)| {
                let nrs: Vec<usize> = input
                    .lines()
                    .take(input.lines().count() - 1)
                    .map(|l| {
                        l.split_ascii_whitespace()
                            .nth(i)
                            .expect("Expected all lines to be same size")
                    })
                    .map(|s| s.parse::<usize>().expect("Expected valid int"))
                    .collect();
                dbg!(&nrs);
                match op {
                    Operation::Plus => nrs.iter().sum::<usize>(),
                    Operation::Multiply => nrs.iter().product::<usize>(),
                }
            })
            .sum();
        Ok(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        Ok("TODO".to_string())
    }
}

enum Operation {
    Plus,
    Multiply,
}

#[cfg(test)]
mod tests {
    use crate::{common::day::Day, days::day6::Day6};

    const EXAMPLE: &str = "123 328  51 64\n45 64  387 23\n 6 98  215 314\n*   +   *   +  ";
    #[test]
    fn part1_example() {
        let day = Day6;
        assert_eq!(day.solve_part1(EXAMPLE).unwrap(), "4277556")
    }
    #[test]
    fn part2_example() {
        let day = Day6;
        assert_eq!(day.solve_part2(EXAMPLE).unwrap(), "3263827")
    }
}
