use aoc2025::*;

fn part1(input: &str) -> i32 {
    let lines = parse_lines(input);
    println!("Lines are {:?}", &lines);

    let dial_size = 100;
    let mut dial_position = 50;
    let mut code = 0;

    for line in lines {
        let direction = line.chars().next().unwrap_or_default();
        let distance: i32 = line.chars().skip(1).collect::<String>().parse().expect("Uh oh");
        //println!("Distance is {:}", distance);

        if 'l' == direction.to_lowercase().next().unwrap_or_default() {
            dial_position -= distance;
        } else if 'r' == direction.to_lowercase().next().unwrap_or_default() {
            dial_position += distance;
        }

        // wrap
        dial_position = dial_position.rem_euclid(dial_size);
        if dial_position == 0 {
            code += 1;
        }

        //println!("Dial is positioned at {:}", dial_position);

    }
    code
}

fn part2(input: &str) -> i32 {
    let lines = parse_lines(input);
    println!("Lines are {:?}", &lines);

    let dial_size = 100;
    let mut dial_position = 50;
    let mut code = 0;

    for line in lines {
        let direction = line.chars().next().unwrap_or_default();
        let distance: i32 = line.chars().skip(1).collect::<String>().parse().expect("Uh oh");
        let mut dir = 'a';
        let mut new_dial_position = 0;

        if 'l' == direction.to_lowercase().next().unwrap_or_default() {
            new_dial_position = dial_position - distance;
            dir = '-';
        } else if 'r' == direction.to_lowercase().next().unwrap_or_default() {
            new_dial_position = dial_position + distance;
            dir = '+';
        }

        let dial_position_wrapped = new_dial_position.rem_euclid(dial_size);
        let mut points_at_0 = 0;

        if dial_position_wrapped == 0 {
            code += 1;
        } else if (new_dial_position < 0 && dial_position != 0) || (new_dial_position >= dial_size && dial_position != dial_size-1) {
            points_at_0 += 1 + (new_dial_position % dial_size).abs() / dial_size;
            code += points_at_0;
        }

        // wrap
        dial_position = dial_position_wrapped;
        if points_at_0 == 0 {
            println!("Dial is rotated {:}{:} to point at {:}", dir, distance, dial_position);
        } else {
            println!("Dial is rotated {:}{:} to point at {:}; during this rotation, it points at 0: {:}", dir, distance, dial_position, points_at_0);
        }

    }
    code
}

fn main() {
    let input = read_input(1);
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 7);
    }
}
