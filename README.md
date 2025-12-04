# Advent of Code 2025 - Rust Solutions

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Project Structure

```
aoc2025/
├── Cargo.toml           # Project configuration with all 25 days as binaries
├── src/
│   ├── lib.rs           # Shared utilities and helper functions
│   └── bin/
│       ├── day01.rs     # Day 1 solution
│       ├── day02.rs     # Day 2 solution
│       └── ...          # (up to day25.rs)
└── inputs/
    ├── day01.txt        # Day 1 input
    ├── day02.txt        # Day 2 input
    └── ...              # (up to day25.txt)
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Running Solutions

Run a specific day's solution:

```bash
cargo run --bin day01
```

Run with release optimizations (faster execution):

```bash
cargo run --release --bin day01
```

### Testing

Run tests for a specific day:

```bash
cargo test --bin day01
```

Run all tests:

```bash
cargo test
```

Run tests for the shared library:

```bash
cargo test --lib
```

## Adding a New Day

1. **Create the solution file**: Copy the template from `src/bin/day01.rs` to `src/bin/dayXX.rs` (where XX is the day number, e.g., `day02.rs`)

2. **Add your input**: Create `inputs/dayXX.txt` and paste your puzzle input

3. **Implement the solution**: Edit the `part1()` and `part2()` functions in your day file

4. **Add tests**: Update the `EXAMPLE` constant with the example from the puzzle and adjust the expected test values

## Utility Functions

The `src/lib.rs` file provides helpful utilities:

- `read_input(day: u8)` - Reads input file for a given day
- `read_input_file(path)` - Reads input from a custom path
- `parse_lines(input)` - Splits input into non-empty lines
- `parse_lines_owned(input)` - Same as above, but returns owned Strings
- `parse_char_grid(input)` - Parses input into a 2D character grid
- `parse_numbers(input)` - Parses lines into a vector of numbers

## Example Solution Template

```rust
use aoc2025::*;

fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

fn main() {
    let input = read_input(1);
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input here
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
```

## Tips

- Use `cargo run --release` for better performance on computationally intensive days
- Write tests using the example inputs from each puzzle to verify your solution
- The utility functions in `lib.rs` can save time on common parsing tasks
- Each day is an independent binary, so they compile separately and can be run in parallel

## Progress

- [*] Day 1
- [**] Day 2
- [*] Day 3
- [ ] Day 4
- [ ] Day 5
- [ ] Day 6
- [ ] Day 7
- [ ] Day 8
- [ ] Day 9
- [ ] Day 10
- [ ] Day 11
- [ ] Day 12
- [ ] Day 13
- [ ] Day 14
- [ ] Day 15
- [ ] Day 16
- [ ] Day 17
- [ ] Day 18
- [ ] Day 19
- [ ] Day 20
- [ ] Day 21
- [ ] Day 22
- [ ] Day 23
- [ ] Day 24
- [ ] Day 25
