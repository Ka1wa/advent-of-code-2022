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
