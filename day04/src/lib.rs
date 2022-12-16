use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    start: u32,
    end: u32,
}

impl TryFrom<&str> for Range {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut pair = value.split("-");
        let start = pair.next().unwrap();
        let end = pair.next().unwrap();

        Ok(Self {
            start: start.parse::<u32>().unwrap(),
            end: end.parse::<u32>().unwrap(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pair {
    first: Range,
    second: Range,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair = s.split(",");
        let first = pair.next().unwrap();
        let second = pair.next().unwrap();

        Ok(Self {
            first: first.try_into()?,
            second: second.try_into()?,
        })
    }
}

impl Pair {
    pub fn fully_overlaps(self) -> bool {
        if self.first.start <= self.second.start && self.first.end >= self.second.end {
            return true;
        }

        if self.second.start <= self.first.start && self.second.end >= self.first.end {
            return true;
        }

        false
    }
    pub fn has_overlap(self) -> bool {
        if self.first.start < self.second.start && self.first.end < self.second.start {
            return false;
        }

        if self.second.start < self.first.start && self.second.end < self.first.start {
            return false;
        }

        true
    }
}

fn input() -> &'static str {
    include_str!("../input.txt")
}

pub fn part_one_naive() -> u32 {
    let mut sum = 0;

    for pair in input()
        .lines()
        .map(|line| line.parse::<Pair>().unwrap()) {
            if pair.fully_overlaps() {
                sum += 1;
            }
    }

    sum
}

pub fn part_two_naive() -> u32 {
    let mut sum = 0;

    for pair in input()
        .lines()
        .map(|line| line.parse::<Pair>().unwrap()) {
        if pair.has_overlap() {
            sum += 1;
        }
    }

    sum
}
