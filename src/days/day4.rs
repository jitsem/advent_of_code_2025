use crate::common::day::Day;
use std::error::Error;

pub struct Day4;

impl Day for Day4 {
    fn get_name(&self) -> &str {
        "Day 4"
    }

    fn get_input_name(&self) -> &str {
        "day4.txt"
    }

    fn get_description(&self) -> &str {
        "Moving of the paper rolls"
    }

    //1372
    fn solve_part1(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let floor = FloorPlan::new(input);
        let mut movable = 0;
        for i in 0..floor.len() {
            if floor.has_paper_at_index(i) && floor.count_surrounding(i) < 4 {
                movable += 1;
            }
        }
        Ok(movable.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>> {
        Ok("TODO".to_string())
    }
}
enum FloorItem {
    Nothing,
    Paper,
}
struct FloorPlan {
    floor: Vec<FloorItem>,
    width: usize,
}

impl FloorPlan {
    fn new(str_plan: &str) -> Self {
        let width = str_plan
            .lines()
            .next()
            .expect("Expect at least on line")
            .len();
        let floor = str_plan
            .lines()
            .flat_map(|l| {
                l.chars().filter_map(|c| match c {
                    '.' => Some(FloorItem::Nothing),
                    '@' => Some(FloorItem::Paper),
                    _ => None,
                })
            })
            .collect();
        Self { floor, width }
    }

    fn len(&self) -> usize {
        self.floor.len()
    }

    fn count_surrounding(&self, idx: usize) -> usize {
        if idx >= self.len() {
            panic!("Provided index is bigger than the floorplan")
        }
        let idx = idx as i64;
        let indexes = [
            idx - self.width as i64,
            idx + self.width as i64,
            if idx % self.width as i64 == 0 {
                -1
            } else {
                idx - 1
            },
            if idx % self.width as i64 == 0 {
                -1
            } else {
                idx + self.width as i64 - 1
            },
            if idx % self.width as i64 == 0 {
                -1
            } else {
                idx - self.width as i64 - 1
            },
            if idx % self.width as i64 == (self.width as i64 - 1) {
                -1
            } else {
                idx + 1
            },
            if idx % self.width as i64 == (self.width as i64 - 1) {
                -1
            } else {
                idx + self.width as i64 + 1
            },
            if idx % self.width as i64 == (self.width as i64 - 1) {
                -1
            } else {
                idx - self.width as i64 + 1
            },
        ];
        indexes
            .iter()
            .filter(|i| **i > 0)
            .map(|i| self.get_at_index(usize::try_from(*i).expect("expected valid usize")))
            .filter_map(|n| match n {
                Some(FloorItem::Paper) => Some(FloorItem::Paper),
                _ => None,
            })
            .count()
    }

    fn has_paper_at_index(&self, idx: usize) -> bool {
        if idx >= self.len() {
            panic!("Provided index is bigger than the floorplan")
        }
        matches!(self.get_at_index(idx), Some(FloorItem::Paper))
    }

    fn get_at_index(&self, idx: usize) -> Option<&FloorItem> {
        self.floor.get(idx)
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::day::Day, days::day4::Day4};
    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn part1_example() {
        let day = Day4;
        assert_eq!(day.solve_part1(EXAMPLE).unwrap(), "13")
    }

    #[test]
    fn part2_example() {
        let day = Day4;
        assert_eq!(day.solve_part2(EXAMPLE).unwrap(), "3121910778619")
    }
}
