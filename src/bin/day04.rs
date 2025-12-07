use std::vec;

use aoc2025::*;

fn get_adjacent_spots(row: usize, column: usize, row_max: usize, column_max: usize) -> Vec<(usize, usize)> {
    use std::cmp::min;
    let mut adjacent_spots: Vec<(usize, usize)> = Vec::new();
    for row_add in -1..=1 {
        if row == 0 && row_add == -1 {
            continue
        }
        let row_prime: i32 = row as i32 + row_add;
        for column_add in -1..=1 {
            if column == 0 && column_add == -1 {
                continue
            }
            if column_add == row_add && column_add == 0 {
                continue
            }
            let column_prime: i32 = column as i32 + column_add;
            if (row_prime < row_max as i32) && (column_prime < column_max as i32) {
                adjacent_spots.push((row_prime as usize, column_prime as usize));
            }
        }
    }
    adjacent_spots    
}


fn part1(input: &str) -> i32 {
    let lines = parse_lines(input);
    //println!("Lines are {:?}", &lines);

    let mut sum = 0;
    let mut vec_de_vecs: Vec<Vec<bool>> = Vec::new();


    for line in lines {
        let chunks = split_into_chunks(&line, line.chars().count());
        match chunks {
            Some(x)=> {
                let mut vec_chunk_bools: Vec<bool> = Vec::new();
                for x in x {
                    if x == "@" {
                        vec_chunk_bools.push(true);
                    } else {
                        vec_chunk_bools.push(false);
                    }
                }
                vec_de_vecs.push(vec_chunk_bools);
            }
            None=>{continue;}
        }
    }
    for (row, line) in vec_de_vecs.iter().enumerate() {
        for (column, roll) in line.iter().enumerate() {
            if *roll {
                let adjacent_spots = get_adjacent_spots(row, column, vec_de_vecs.iter().count(), line.iter().count());
                let mut num_of_adjacent_roles: i32 = 0;
                for (rows, columns) in adjacent_spots {
                    let is_roll: bool = vec_de_vecs[rows][columns];
                    if is_roll {
                        num_of_adjacent_roles += 1;
                    }
                }
                if num_of_adjacent_roles < 4 {
                    sum += 1;
                    print!("x");
                } else {
                    print!("@");
                }
            } else {
                print!(".")
            }
        }
        println!("");
    }
    sum
}

fn part2(input: &str) -> i64 {

    // Remove the indices from flattened vector that correspond to the 3 smallest values
    // Then select all indices that remain 
    0
}

fn main() {
    let input = read_input(4);
    
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
