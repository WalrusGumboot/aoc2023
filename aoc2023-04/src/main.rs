use std::{collections::{HashSet, HashMap}, fs};

fn convert_numbers(s: &str) -> HashSet<u32> {
    s.split(' ')
        .filter(|num| num != &"")
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<HashSet<_>>()
}

#[derive(Clone)]
struct Scratchcard {
    winning: HashSet<u32>,
    have: HashSet<u32>,

}

fn main() {
    let cards = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let (_, data) = line.split_once(':').unwrap();
            let (winning_raw, have_raw) = data.split_once('|').unwrap();

            let winning = convert_numbers(winning_raw);
            let have = convert_numbers(have_raw);

            Scratchcard { winning, have }
        })
        .collect::<Vec<_>>();

    let part_one = cards
        .iter()
        .map(|card| {
            let matches = card.winning.intersection(&card.have).count();

            // println!("{matches}");

            if matches == 0 {
                0
            } else {
                2u32.pow(matches as u32 - 1u32)
            }
        })
        .sum::<u32>();
    println!("{part_one}");

    // part two
    let mut extra_cards = (0..cards.len()).map(|idx| (idx, 1u32)).collect::<HashMap<_, _>>();
    for (idx, card) in cards.into_iter().enumerate() {
        let copies = *extra_cards.get(&idx).unwrap();
        let matches = card.winning.intersection(&card.have).count();
        
        for idx_to_be_added_to in idx + 1..=idx + matches {
            *extra_cards.get_mut(&idx_to_be_added_to).unwrap() += copies;
        }
    }

    let part_two = extra_cards.values().sum::<u32>();

    println!("{part_two}")
}
