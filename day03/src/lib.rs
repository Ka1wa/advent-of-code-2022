use std::collections::HashMap;
use itertools::Itertools;

fn char_to_priority(char: char) -> u32 {
    match char {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

pub fn part_one_naive() -> u32 {
    let mut sum = 0;

    for line in input().lines() {
        let rucksack_items = line.len();
        let compartment_items = rucksack_items / 2;
        let left: String = line.chars().take(compartment_items).collect();
        let right: String = line.chars().skip(compartment_items).take(compartment_items).collect();

        let mut left_compartment: HashMap<char, u32> = HashMap::new();

        for char in left.chars() {
            *left_compartment.entry(char).or_insert(0) += 1;
        }

        for char in right.chars() {
            if left_compartment.contains_key(&char) {
                sum += char_to_priority(char);
                break;
            }
        }
    }

    sum
}

pub fn part_two_naive() -> u32 {
    let mut sum = 0;

    for group in &input().lines().chunks(3) {
        let mut rucksacks: HashMap<char, u32> = HashMap::new();

        for line in group {
            let mut chars = line.chars().collect::<Vec<char>>();
            chars.sort();
            chars.dedup();

            for char in chars {
                *rucksacks.entry(char).or_insert(0) += 1;
            }
        }

        for (key, value) in rucksacks.into_iter() {
            if value == 3 {
                sum += char_to_priority(key);
                break;
            }
        }
    }

    sum
}
