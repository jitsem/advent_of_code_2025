pub struct Util;
impl Util {
    pub fn extract_all_numbers_from_str_range(
        input: &str,
        seperator: &str,
    ) -> impl Iterator<Item = usize> {
        input.split(seperator).flat_map(|range| {
            let (lower, upper) = range.split_once('-').expect("Expected x-y");
            let lower = lower.parse::<usize>().expect("Expected x");
            let upper = upper.parse::<usize>().expect("Expected y");
            lower..=upper
        })
    }
}
