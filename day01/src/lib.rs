pub fn naive() -> u32 {
    let mut highest: u32 = 0;
    let mut current: u32 = 0;

    for line in include_str!("../input.txt").lines() {
        if line.is_empty() {
            if current > highest {
                highest = current;
            }

            current = 0;
        } else {
            let calories = line.parse::<u32>().unwrap();

            current += calories;
        }
    }

    highest
}

pub fn itertools() -> u32 {
    // https://fasterthanli.me/series/advent-of-code-2022/part-1#the-problem-statement
    use itertools::Itertools;
    use std::cmp::Reverse;

    include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u32>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u32>()
}