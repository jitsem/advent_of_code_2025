use crate::common::day::Day;
use std::error::Error;

enum SafeDialAction {
    Left(isize),
    Rigth(isize),
}

struct SafeDial {
    current_pos: isize,
}

struct SafeDialResult {
    current_pos: isize,
    times_passed_zero: isize,
}

impl SafeDial {
    fn new() -> Self {
        SafeDial { current_pos: 50 }
    }
    fn turn(&mut self, action: &SafeDialAction) -> SafeDialResult {
        let offset = match action {
            SafeDialAction::Left(ticks) => -*ticks,
            SafeDialAction::Rigth(ticks) => *ticks,
        };

        if self.current_pos == 0 && offset < 0 {
            self.current_pos = 100;
        }

        self.current_pos += offset;
        let mut times_passed_zero = (self.current_pos / 100).abs();

        if self.current_pos < 1 {
            times_passed_zero += 1;
        }

        self.current_pos %= 100;
        if self.current_pos < 0 {
            self.current_pos += 100;
        }

        SafeDialResult {
            current_pos: self.current_pos,
            times_passed_zero,
        }
    }
}

pub struct Day1;
impl Day for Day1 {
    //Result 1034
    fn solve_part1(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let mut zero_count = 0;
        let mut dial = SafeDial::new();
        for line in input.lines() {
            let mut chars = line.chars();
            let action: Result<SafeDialAction, Box<dyn Error>> = match chars.next() {
                Some('L') => Ok(SafeDialAction::Left(chars.as_str().parse::<isize>()?)),
                Some('R') => Ok(SafeDialAction::Rigth(chars.as_str().parse::<isize>()?)),
                Some(_) => Err(("Encountered unknown letter").into()),
                None => Err("Encountered empty line".into()),
            };
            let action = action?;
            if dial.turn(&action).current_pos == 0 {
                zero_count += 1
            }
        }
        Ok(zero_count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let mut times_past_zero = 0;
        let mut dial = SafeDial::new();
        for line in input.lines() {
            let mut chars = line.chars();
            let action: Result<SafeDialAction, Box<dyn Error>> = match chars.next() {
                Some('L') => Ok(SafeDialAction::Left(chars.as_str().parse::<isize>()?)),
                Some('R') => Ok(SafeDialAction::Rigth(chars.as_str().parse::<isize>()?)),
                Some(_) => Err(("Encountered unknown letter").into()),
                None => Err("Encountered empty line".into()),
            };
            let action = action?;
            let action_result = dial.turn(&action);
            times_past_zero += action_result.times_passed_zero;
        }
        Ok(times_past_zero.to_string())
    }

    fn get_name(&self) -> String {
        "Day 1".into()
    }

    fn get_input_name(&self) -> String {
        "day1.txt".into()
    }

    fn get_description(&self) -> String {
        "Password troubles".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::day::Day, days::day1::Day1};

    #[test]
    fn part1_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let day = Day1;
        let res = day.solve_part1(input).unwrap();
        assert_eq!(res, "3")
    }
    #[test]
    fn part2_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let day = Day1;
        let res = day.solve_part2(input).unwrap();
        assert_eq!(res, "6")
    }
}
