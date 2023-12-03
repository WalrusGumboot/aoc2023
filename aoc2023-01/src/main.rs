use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    
    // part one -- just numerical
    let part_one: u32 = input.lines().map(|line| {
        let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
        let tens_place = digits.first().unwrap_or(&'0').to_digit(10).unwrap();
        let ones_place = digits.last().unwrap_or(&'0').to_digit(10).unwrap();
        10 * tens_place + ones_place
    }).sum();
    println!("{part_one}");

    // part two -- also some text in there
    let part_two: u32 = input.lines().map(|line| {
        let mut digits: Vec<u32> = Vec::new();
        for (idx, c) in line.char_indices() {
            if c.is_numeric() {
                digits.push(c.to_digit(10).unwrap())
            } else if c.is_alphabetic() {
                let substr = line.chars().skip(idx).collect::<String>();
                if substr.starts_with("one")   { digits.push(1); } else
                if substr.starts_with("two")   { digits.push(2); } else
                if substr.starts_with("three") { digits.push(3); } else
                if substr.starts_with("four")  { digits.push(4); } else
                if substr.starts_with("five")  { digits.push(5); } else
                if substr.starts_with("six")   { digits.push(6); } else
                if substr.starts_with("seven") { digits.push(7); } else
                if substr.starts_with("eight") { digits.push(8); } else
                if substr.starts_with("nine")  { digits.push(9); } 
            }
        }
        
        let tens_place = digits.first().unwrap_or(&0);
        let ones_place = digits.last().unwrap_or(&0);

        10 * tens_place + ones_place
    }).sum();
    println!("{part_two}");
}
