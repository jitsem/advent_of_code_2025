use std::error::Error;

pub trait Day {
    fn get_name(&self) -> &str;
    fn get_input_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn solve_part1(&self, input: &str) -> Result<String, Box<dyn Error>>;
    fn solve_part2(&self, input: &str) -> Result<String, Box<dyn Error>>;
}
