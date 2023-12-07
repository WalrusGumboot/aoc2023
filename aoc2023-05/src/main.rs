use std::{collections::HashMap, fs, ops::Range};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct Mapping {
    source_start: u64,
    dest_start: u64,
    source: Range<u64>,
    dest: Range<u64>,
}

impl Mapping {
    fn new(dest_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        Self {
            source_start: source_range_start,
            dest_start: dest_range_start,
            source: source_range_start..source_range_start + range_length,
            dest: dest_range_start..dest_range_start + range_length,
        }
    }

    /// Constructs an explicit lookup table
    fn explicit(&self) -> HashMap<u64, u64> {
        self.source.clone().zip(self.dest.clone()).collect()
    }

    fn map(&self, value: u64) -> u64 {
        assert!(self.source.contains(&value));
        let offset = value - self.source_start;
        self.dest_start + offset
    }
}

// fn unify(mappings: Vec<Mapping>) -> HashMap<u64, u64> {
//     mappings
//         .iter()
//         .map(|mapping| mapping.explicit())
//         .reduce(|mut acc, curr| {
//             acc.extend(curr);
//             acc
//         })
//         .unwrap()
// }

fn take_into_mapping_collection<'a, T: Iterator<Item = &'a str>>(
    lines_iter: &mut T,
) -> Vec<Mapping> {
    // remove the descriptive text line
    // lines_iter.next().unwrap();
    // unify(
    lines_iter
        .take_while(|line| line.chars().next().unwrap().is_numeric())
        .map(|line| {
            // println!("{line}");
            let mut line_numbers = line.split(' ').map(|num| num.parse::<u64>().unwrap());
            Mapping::new(
                line_numbers.next().unwrap(), // takes the first number
                line_numbers.next().unwrap(), // takes the second number
                line_numbers.next().unwrap(), // takes the third number
            )
        })
        .collect()
    // )
}

fn traverse_sequence(sequence: &Vec<Vec<Mapping>>, input_seed: u64) -> u64 {
    let mut lookup_key = input_seed;
    for lookuptable in sequence {
        let maybe_relevant_submapping = lookuptable
            .iter()
            .find(|mapping| mapping.source.contains(&lookup_key));
        if let Some(submapping) = maybe_relevant_submapping {
            lookup_key = submapping.map(lookup_key);
        }
    }
    lookup_key
}

fn main() {
    let raw_input = fs::read_to_string("./input.txt").unwrap();
    let mut raw_input_lines_iter = raw_input.lines().filter(|line| line != &"");

    let seeds = raw_input_lines_iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    raw_input_lines_iter.next().unwrap();

    // println!("seed to soil");
    let seed_to_soil = take_into_mapping_collection(&mut raw_input_lines_iter);
    // println!("soil_to_fertilizer");
    let soil_to_fertilizer = take_into_mapping_collection(&mut raw_input_lines_iter);
    // println!("fertilizer_to_water");
    let fertilizer_to_water = take_into_mapping_collection(&mut raw_input_lines_iter);
    // println!("water_to_light");
    let water_to_light = take_into_mapping_collection(&mut raw_input_lines_iter);
    // println!("light_to_temperature");
    let light_to_temperature = take_into_mapping_collection(&mut raw_input_lines_iter);
    // println!("temperature_to_humidity");
    let temperature_to_humidity = take_into_mapping_collection(&mut raw_input_lines_iter);
    // println!("humidity_to_location");
    let humidity_to_location = take_into_mapping_collection(&mut raw_input_lines_iter);

    let sequence = vec![
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    let mut destinations = seeds
        .clone()
        .iter()
        .map(|seed| traverse_sequence(&sequence, *seed))
        .collect::<Vec<_>>();
    destinations.sort();

    let part_one = destinations[0];
    println!("{part_one}");

    // part two

    let seed_ranges = seeds
        .iter()
        .as_slice()
        .chunks_exact(2)
        .map(|chunk_slice| chunk_slice[0]..chunk_slice[0] + chunk_slice[1]);

    // collecting all values in a huge Vec proved too memory-intensive
    // so storing them to disk is the better way to go

    // ok nvm this is a full 42 GiB of data lmao, GNU sort cannot handle this on my laptop
    // however I can't be arsed to think about this problem more thoroughly
    let mut file = File::create("destinations.txt").unwrap();

    for seed_range in seed_ranges {
        for seed_value in seed_range { 
            // println!("checking seed {seed_value}");
            writeln!(file, "{} {}", seed_value, traverse_sequence(&sequence, seed_value)).unwrap();
        }
    }
}
