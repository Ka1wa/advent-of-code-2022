use std::env;
use std::fs;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let input_file = current_dir.join("input.txt");
    let Ok(input) = fs::read_to_string(input_file) else {
        panic!("Couldn't read input file");
    };

    let mut highest: u32 = 0;
    let mut highest_elf: u32 = 0;
    let mut current: u32 = 0;
    let mut current_elf: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current > highest {
                highest = current;
                highest_elf = current_elf;
            }

            current = 0;
            current_elf += 1;
        } else {
            let calories = line.parse::<u32>().unwrap();

            current += calories;
        }
    }

    println!("The elf with the most calories is elf #{} with a total of: {} calories", highest_elf + 1, highest);
}
