use std::{collections::HashMap, fs};

fn rev_tuple<T>(val: (T, T)) -> (T, T) {
    (val.1, val.0)
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    // part one -- indices of games possible with >= 12 red, >= 13 green and >= 14 blue cubes 
    let part_one: u32 = input.lines().filter_map(|game| {
        let (game_idx_raw, game_desc) = game.split_once(':').unwrap();
        let game_idx = game_idx_raw
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let runs = game_desc.split("; ").map(|s| s.trim()).collect::<Vec<_>>();

        for run in runs {
            let colours = run
                .split(", ")
                .map(|s| rev_tuple(s.split_once(' ').unwrap()))
                .map(|(k, v)| (k, v.parse::<usize>().unwrap()))
                .collect::<HashMap<&str, usize>>();

            if let Some(_r @ 13..) = colours.get("red")   { return None; }
            if let Some(_g @ 14..) = colours.get("green") { return None; }
            if let Some(_b @ 15..) = colours.get("blue")  { return None; }
        }

        Some(game_idx)
    }).sum();

    println!("{part_one}");

    // part two -- sum of product of minimum amount of cubes
    let part_two: u32 = input.lines().map(|game| {
        let (_, game_desc) = game.split_once(':').unwrap();

        let runs = game_desc.split("; ").map(|s| s.trim()).collect::<Vec<_>>();


        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;

        for run in runs {
            let colours = run
                .split(", ")
                .map(|s| rev_tuple(s.split_once(' ').unwrap()))
                .map(|(k, v)| (k, v.parse::<u32>().unwrap()))
                .collect::<HashMap<&str, u32>>();

            if let Some(r) = colours.get("red")   { min_red   = min_red.max(*r); }
            if let Some(g) = colours.get("green") { min_green = min_green.max(*g); }
            if let Some(b) = colours.get("blue")  { min_blue  = min_blue.max(*b); }
        }

        min_red * min_green * min_blue
    }).sum();
    println!("{part_two}");
}
