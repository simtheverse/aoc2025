use std::fs;
use std::path::Path;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

/// Read input file from a custom path
pub fn read_input_file<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path.as_ref())
        .unwrap_or_else(|_| panic!("Failed to read input file: {:?}", path.as_ref()))
}

/// Parse input into lines, filtering out empty lines
pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

/// Parse input into lines as owned Strings
pub fn parse_lines_owned(input: &str) -> Vec<String> {
    input.lines().filter(|line| !line.is_empty()).map(String::from).collect()
}

/// Parse input into a grid of characters
pub fn parse_char_grid(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

/// Parse input into numbers (one per line)
pub fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse().expect("Failed to parse number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "line1\nline2\n\nline3\n";
        let lines = parse_lines(input);
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_parse_char_grid() {
        let input = "abc\ndef\n";
        let grid = parse_char_grid(input);
        assert_eq!(grid, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }

    #[test]
    fn test_parse_numbers() {
        let input = "1\n2\n3\n";
        let numbers: Vec<i32> = parse_numbers(input);
        assert_eq!(numbers, vec![1, 2, 3]);
    }
}
