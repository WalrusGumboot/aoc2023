use std::fs;

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u32,
    is_next_to_char: bool,
}

fn check_neighbours(grid: &Vec<Vec<Option<char>>>, row: usize, col: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    let x_checks: &[i32] = if col > 0 && col < width - 1 {
        &[-1, 0, 1]
    } else if col == 0 {
        &[0, 1]
    } else {
        &[-1, 0]
    };
    let y_checks: &[i32] = if row > 0 && row < height - 1 {
        &[-1, 0, 1]
    } else if row == 0 {
        &[0, 1]
    } else {
        &[-1, 0]
    };

    // println!("coord: ({row}, {col}), x offs: {x_checks:?}; y offs: {y_checks:?}");

    for x in x_checks {
        for y in y_checks {
            let to_check = grid[(row as i32 + y) as usize][(col as i32 + x) as usize];
            if let Some(c) = to_check {
                if !c.is_numeric() {
                    return true;
                }
            }
        }
    }

    false
}

// // used in part two
// enum Entry {
//     Digit(u8),
//     Gear(u8) // number of neighbours
// }

fn main() {
    let raw_input = fs::read_to_string("./input.txt").unwrap();
    let grid: Vec<Vec<Option<char>>> = raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '.' { None } else { Some(c) })
                .collect()
        })
        .collect();


    // part one -- sum of all numbers

    // now, we make the numbers contiguous and check if they're next to non-period characters
    let mut numbers = Vec::new();
    let mut working_number = None;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // print!("at ({row}, {col}) -> ");
            if let Some(c) = grid[row][col] {
                // print!("hit char -> ");
                if c.is_numeric() {
                    // print!("hit num -> ");
                    match working_number {
                        None => {
                            working_number = Some(Number {
                                value: c.to_digit(10).unwrap(),
                                is_next_to_char: check_neighbours(&grid, row, col),
                            });
                            // println!("had no working number so made one: {:?}", working_number.unwrap());
                        }
                        Some(mut num) => {
                            num.value *= 10;
                            num.value += c.to_digit(10).unwrap();
                            num.is_next_to_char |= check_neighbours(&grid, row, col);
                            working_number = Some(num);
                            // println!("had a working number so made it: {:?}", working_number.unwrap());
                        }
                    }
                } else {
                    // print!("hit nonnum -> ");
                    if let Some(num) = working_number {
                        // print!("have a working number -> ");
                        if num.is_next_to_char {
                            numbers.push(num);
                            // print!("added it -> ");
                        }
                        working_number = None;
                        // println!("made working number none");
                    } else {
                        // println!("nothing");
                    }
                }
            } else {
                // we're looking at a None
                // print!("hit none -> ");
                if let Some(num) = working_number {
                    // print!("have a working number -> ");
                    if num.is_next_to_char {
                        numbers.push(num);
                        // print!("added it -> ");
                    }
                    working_number = None;
                    // println!("made working number none");
                } else {
                    // println!("nothing");
                }
            }
        }

        if let Some(num) = working_number {
            // print!("have a working number -> ");
            if num.is_next_to_char {
                numbers.push(num);
                // print!("added it -> ");
            }
            working_number = None;
            // println!("made working number none");
        } else {
            // println!("nothing");
        }
    }

    if let Some(num) = working_number {
        // print!("have a working number -> ");
        if num.is_next_to_char {
            numbers.push(num);
            // print!("added it -> ");
        }
        working_number = None;
        // println!("made working number none");
    } else {
        // println!("nothing");
    }

    let part_one: u32 = numbers.iter().map(|n| n.value).sum();
    println!("{part_one}");

    // // part two -- sum of "gear ratios"

    // let mut adapted_grid = grid.clone().iter().map(|row| row.iter().map(|c| {
    //     if let Some(ch) = c {
    //         if ch == &'*' {
    //             Some(Entry::Gear(0))
    //         } else if ch.is_numeric() {
    //             Some(Entry::Digit(ch.to_digit(10).unwrap().try_into().unwrap()))
    //         } else { None }
    //     } else { None }
    // }).collect::<Vec<_>>()).collect::<Vec<_>>();

    // for row in 0..adapted_grid.len() {
    //     for col in 0..adapted_grid[0].len() {

    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_neighbours_test() {
        let grid = vec![
            vec![Some('1'), Some('2'), None],
            vec![None, None, Some('*')],
            vec![None, None, Some('1')],
        ];

        assert!(!check_neighbours(&grid, 0, 0));
        assert!(check_neighbours(&grid, 0, 1));
        assert!(check_neighbours(&grid, 2, 2));
        assert!(!check_neighbours(&grid, 1, 0));
    }
}
