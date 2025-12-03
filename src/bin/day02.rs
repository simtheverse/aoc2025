use aoc2025::*;

fn part1(input: &str) -> i64 {
    let lines = parse_delimited(input, ',');
    println!("Lines are {:?}", &lines);

    let mut sum = 0;

    for line in lines {
        let range_bounds = parse_delimited(line, '-');
        let range_low : i64 = range_bounds[0].parse().expect("uh oh");
        let range_high : i64 = range_bounds[1].parse().expect("uh oh");

        let range = (range_low..=range_high);
        for i in range {
            let i_to_str :String = i.to_string();
            let count =  i_to_str.chars().count();
            if count % 2 == 0 {
                let mid = i_to_str.chars().count() / 2;
                let left: String = i_to_str.chars().take(mid).collect();
                let right: String = i_to_str.chars().skip(mid).collect();
                if left.parse::<i64>() == right.parse::<i64>() {
                    sum += i;
                }
            }
        }


    }
    sum
}

fn part2(input: &str) -> i64 {
    let lines = parse_delimited(input, ',');
    println!("Lines are {:?}", &lines);

    let mut sum = 0;

    for line in lines {
        let range_bounds = parse_delimited(line, '-');
        let range_low : i64 = range_bounds[0].parse().expect("uh oh");
        let range_high : i64 = range_bounds[1].parse().expect("uh oh");

        let range = (range_low..=range_high);
        'outer: for i in range {
            let i_to_str :String = i.to_string();
            let count =  i_to_str.chars().count();
            'inner: for j in 2..=count {
                let chunks = split_into_chunks(&i_to_str, j);
                match chunks {
                    Some(x)=> {
                        if x.is_empty() {
                            continue 'inner;
                        }
                        let first = &x[0];
                        if x.iter().all(|item| item == first) {
                            sum += i;
                            println!("match: {:}. {:} chunks.", i, j);
                            continue 'outer;
                        }
                    }
                    None=> {continue 'inner;}
                }
            }
        }


    }
    sum
}

fn main() {
    let input = read_input(2);
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
