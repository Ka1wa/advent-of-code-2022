use std::cmp::Ordering;

// A: Rock (1)
// B: Paper (2)
// C: Scissors (3)
#[derive(Eq, Debug, PartialOrd, PartialEq, Copy, Clone)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

// X: Lose
// Y: Draw
// Z: Win
#[derive(Copy, Clone)]
enum Result {
    Lose,
    Draw,
    Win
}

impl Ord for Sign {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Sign::Rock, Sign::Rock) => Ordering::Equal,
            (Sign::Rock, Sign::Paper) => Ordering::Greater,
            (Sign::Rock, Sign::Scissors) => Ordering::Less,
            (Sign::Paper, Sign::Rock) => Ordering::Less,
            (Sign::Paper, Sign::Paper) => Ordering::Equal,
            (Sign::Paper, Sign::Scissors) => Ordering::Greater,
            (Sign::Scissors, Sign::Rock) => Ordering::Greater,
            (Sign::Scissors, Sign::Paper) => Ordering::Less,
            (Sign::Scissors, Sign::Scissors) => Ordering::Equal,
        }
    }
}

impl TryInto<Sign> for &str {
    type Error = ();

    fn try_into(self) -> std::result::Result<Sign, Self::Error> {
        match self {
            "A" => Ok(Sign::Rock),
            "B" => Ok(Sign::Paper),
            "C" => Ok(Sign::Scissors),
            _ => Err(())
        }
    }
}

impl TryInto<Result> for &str {
    type Error = ();

    fn try_into(self) -> std::result::Result<Result, Self::Error> {
        match self {
            "X" => Ok(Result::Lose),
            "Y" => Ok(Result::Draw),
            "Z" => Ok(Result::Win),
            _ => Err(())
        }
    }
}

fn result_to_sign(result: &Result, opponent_sign: &Sign) -> Sign {
    match (result, opponent_sign) {
        (Result::Lose, Sign::Rock) => Sign::Scissors,
        (Result::Lose, Sign::Paper) => Sign::Rock,
        (Result::Lose, Sign::Scissors) => Sign::Paper,
        (Result::Draw, Sign::Rock) => Sign::Rock,
        (Result::Draw, Sign::Paper) => Sign::Paper,
        (Result::Draw, Sign::Scissors) => Sign::Scissors,
        (Result::Win, Sign::Rock) => Sign::Paper,
        (Result::Win, Sign::Paper) => Sign::Scissors,
        (Result::Win, Sign::Scissors) => Sign::Rock,
    }
}

pub fn naive() -> u32 {
    let mut total_score = 0;

    for line in include_str!("../input.txt").lines() {
        let mut round = line.split(" ");

        let opponent_sign: Sign = round.next().unwrap().try_into().unwrap();
        let result: Result = round.next().unwrap().try_into().unwrap();
        let player_sign = result_to_sign(&result, &opponent_sign);

        let cmp = opponent_sign.cmp(&player_sign);

        let round_score = match cmp {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6
        };
        let sign_score = player_sign as u32;

        total_score += sign_score + round_score;
    }

    total_score
}
