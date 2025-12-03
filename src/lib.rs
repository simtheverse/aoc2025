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

/// Parse a delimited string slice into a vector of string slices
pub fn parse_delimited(input: &str, delimiter: char) -> Vec<&str> {
    input.split(delimiter).map(|s| s.trim()).collect()
}

/// Parse a delimited string slice into a vector of parsed values
pub fn parse_delimited_as<T>(input: &str, delimiter: char) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .split(delimiter)
        .map(|s| s.trim().parse().expect("Failed to parse value"))
        .collect()
}

/// Split a string into N equal-sized chunks (by character count, handles Unicode correctly)
/// Returns None if the string cannot be evenly divided into N chunks
pub fn split_into_chunks(input: &str, n: usize) -> Option<Vec<String>> {
    if n == 0 {
        return None;
    }
    
    let char_count = input.chars().count();
    if char_count % n != 0 {
        return None;
    }
    
    let chunk_size = char_count / n;
    let mut chunks = Vec::new();
    let mut chars = input.chars();
    
    for _ in 0..n {
        let chunk: String = chars.by_ref().take(chunk_size).collect();
        chunks.push(chunk);
    }
    
    Some(chunks)
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

    #[test]
    fn test_parse_delimited() {
        let input = "a-b-c-d";
        let parts = parse_delimited(input, '-');
        assert_eq!(parts, vec!["a", "b", "c", "d"]);
        
        let input_pipe = "a | b | c";
        let parts = parse_delimited(input_pipe, '|');
        assert_eq!(parts, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_delimited_as() {
        let input = "1-2-3-4";
        let numbers: Vec<i32> = parse_delimited_as(input, '-');
        assert_eq!(numbers, vec![1, 2, 3, 4]);
        
        let input_pipe = "10 | 20 | 30";
        let numbers: Vec<i32> = parse_delimited_as(input_pipe, '|');
        assert_eq!(numbers, vec![10, 20, 30]);
    }

    #[test]
    fn test_split_into_chunks() {
        // Split in half
        let input = "abcdef";
        let chunks = split_into_chunks(input, 2);
        assert_eq!(chunks, Some(vec!["abc".to_string(), "def".to_string()]));
        
        // Split into 3 chunks
        let input = "123456789";
        let chunks = split_into_chunks(input, 3);
        assert_eq!(chunks, Some(vec!["123".to_string(), "456".to_string(), "789".to_string()]));
        
        // Cannot split evenly - returns None
        let input = "abcde";
        let chunks = split_into_chunks(input, 2);
        assert_eq!(chunks, None);

        let input = "abcde";
        let chunks = split_into_chunks(input, 5);
        assert_eq!(chunks, Some(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]));
        
        
        // Unicode handling
        let input = "🎄🎅🎁🎉🎊🎈";
        let chunks = split_into_chunks(input, 2);
        assert_eq!(chunks, Some(vec!["🎄🎅🎁".to_string(), "🎉🎊🎈".to_string()]));
        
        // Edge case: n = 0
        let input = "abc";
        let chunks = split_into_chunks(input, 0);
        assert_eq!(chunks, None);
        
        // Edge case: split into 1 chunk
        let input = "hello";
        let chunks = split_into_chunks(input, 1);
        assert_eq!(chunks, Some(vec!["hello".to_string()]));
    }
}
