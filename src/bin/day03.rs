use aoc2025::*;

fn count_unique_items<T: Ord+Clone>(vec: &mut Vec<T>) -> usize {
    vec.sort();
    vec.dedup();
    vec.len()
}

fn part1(input: &str) -> i32 {
    let lines = parse_lines(input);
    //println!("Lines are {:?}", &lines);

    let mut sum = 0;

    for line in lines {
        let chunks = split_into_chunks(&line, line.chars().count());
        match chunks {
            Some(x)=> {
                let numbers: Vec<i32> = x.into_iter().map(|y| y.parse::<i32>().expect("uh oh")).collect();
                //println!("numbers: {:?}", numbers);
                // sort numbers from highest to lowest
                let mut sorted_nums: Vec<i32> = numbers.clone();
                sorted_nums.sort();
                sorted_nums.dedup();
                let sorted_nums : Vec<i32> = sorted_nums;

                let unique_count = sorted_nums.len();
                //println!("sorted_nums is {:?}", sorted_nums);
                let mut vec_of_vecs: Vec<Vec<i32>> = Vec::new();



                // get vec of ordered indexes for each of the sorted numbers. push on vec_of_vecs
                for x in sorted_nums.iter().rev() {
                    let mut indexes: Vec<i32> = Vec::new();
                    for (index, value) in numbers.iter().enumerate() {
                        if *value == *x {
                            indexes.push(index as i32)
                        }
                    }
                    vec_of_vecs.push(indexes.clone())
                }
                //println!("vec_of_vecs is {:?}", vec_of_vecs);
                // flatten vec_of_vecs (append .n to .n-1) into 1 flat_vec
                let mut flat_indexes: Vec<i32> = vec_of_vecs.into_iter().flatten().collect();

                // remove all indicies numerically less than the value of the first item in flat_indexes
                let first_val = *flat_indexes.get(0).expect("uh oh");
                if flat_indexes.iter().any(|&x| x > first_val) {
                    flat_indexes.retain(|&x| x >= first_val);
                }
            
                // new vec<chars> for selection
                //println!("flat indexes is: {:?}", flat_indexes);
                //println!("is: {:} and {:}", flat_indexes[0], flat_indexes[1]);

                let mut selection_index = vec![flat_indexes[0], flat_indexes[1]];
                selection_index.sort();
                let selection: Vec<char> = vec![char::from_digit(numbers[selection_index[0] as usize] as u32, 10).expect("uh oh"), char::from_digit(numbers[selection_index[1] as usize] as u32, 10).expect("uh oh")];
                //println!("selection is {:?}", selection);

                sum += selection.into_iter().collect::<String>().parse::<i32>().expect("uh oh");


            }
            None=>{continue;}
        }
    }
    sum
}

fn part2(input: &str) -> i64 {

    // Remove the indices from flattened vector that correspond to the 3 smallest values
    // Then select all indices that remain 
    0
}

fn main() {
    let input = read_input(3);
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
